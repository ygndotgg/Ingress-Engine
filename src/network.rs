use core::panic;
use std::time::Duration;

use crate::config::Config;
use bytes::{Bytes, BytesMut};
use log::{error, info, warn};
use thiserror::Error;
use tokio::{
    io::{self, AsyncReadExt},
    net::{TcpListener, TcpStream},
    time::sleep,
};

pub struct NetworkConnector {
    listener: TcpListener,
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("More bytes required to determine length")]
    Incomplete,
    #[error("Malformed variable byte integer")]
    Malformed,
}

impl NetworkConnector {
    pub async fn new(addr: &str) -> Self {
        let listener = match TcpListener::bind(addr).await {
            Ok(l) => l,
            Err(e) => panic!("{:?}", e),
        };
        Self { listener }
    }
    pub async fn run(&self) {
        loop {
            match self.listener.accept().await {
                Ok((stream, socket_addr)) => {
                    info!("New connection from :{}", socket_addr);
                    tokio::spawn(async {
                        let mut connection = Connection::new(stream);
                        connection.process().await;
                    });
                }
                Err(e) => {
                    if is_emfile(&e) {
                        warn!("EMFILE: Too many open files. Reactor sleeping for 10ms");
                        sleep(Duration::from_millis(10)).await;
                    } else {
                        error!("Accept error: {:?}", e);
                        sleep(Duration::from_millis(5)).await;
                    }
                }
            }
        }
    }
}

fn is_emfile(err: &io::Error) -> bool {
    match err.raw_os_error() {
        // 23 -- process has too many open files
        // 23 System wide limit of open files reached
        Some(code) => code == 24 || code == 23,
        None => false,
    }
}

struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    fn new(stream: TcpStream) -> Connection {
        let buf_size = Config::global().tcp_recv_buf_size;
        Connection {
            stream: stream,
            buffer: BytesMut::with_capacity(buf_size),
        }
    }
    async fn process(&mut self) {
        loop {
            match self.stream.read_buf(&mut self.buffer).await {
                Ok(0) => return,
                Ok(_n) => {
                    self.buffer.clear();
                }
                Err(e) => {
                    panic!("{:?}", e)
                }
            }
        }
    }

    pub async fn read_packet(&mut self) -> io::Result<Option<Bytes>> {
        loop {
            match self.parse_frame() {
                Ok(d) => match d {
                    // full packet found
                    Some(s) => return Ok(Some(s)),
                    None => {
                        // need more data
                    }
                },
                Err(DecodeError::Malformed) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Malformed MQTT Variable Integer",
                    ))
                }
                Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Protcol Error")),
            }
            if self.buffer.capacity() - self.buffer.len() < 1024 {
                self.buffer.reserve(1024);
            }
            let n = self.stream.read_buf(&mut self.buffer).await?;
            if n == 0 {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::ConnectionReset,
                        "Connection has been reset by peer",
                    ));
                }
            }
        }
    }
    fn parse_frame(&mut self) -> Result<Option<Bytes>, DecodeError> {
        if self.buffer.len() < 2 {
            return Ok(None);
        }
        let mut len: usize = 0;
        let mut multiplier = 1;
        let mut length_byte_count = 0;
        let mut done = false;

        // if the buffer len is less than 5 it works
        for i in 1..std::cmp::min(self.buffer.len(), 5) {
            let byte = self.buffer[i];
            let low_bytes = byte & 0x7f;
            len += (low_bytes as usize) * multiplier;

            multiplier *= 128;
            length_byte_count += 1;
            if (byte & 0x80) == 0 {
                done = true;
                break;
            }
        }
        // if we had read 4 bytes and last byte MSB is 1 so it is still continous (Malformed).
        if !done && length_byte_count == 4 {
            return Err(DecodeError::Malformed);
        }
        // We didnt found the end of the length field yet
        if !done {
            return Ok(None);
        }
        let total_size = 1 + length_byte_count + len;
        if self.buffer.len() < total_size {
            //We have the length, but not enough data for the payload yet so we fire for entire
            return Ok(None);
        }
        Ok(Some(self.buffer.split_to(total_size).freeze()))
    }
}
