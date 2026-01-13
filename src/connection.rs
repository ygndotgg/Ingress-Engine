use core::panic;
use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use crate::config::Config;

pub fn conn() {
    let nagle = Config::global().tcp_nodelay;
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    listener
        .set_nonblocking(true)
        .expect("Cannot Set Non-blocking");
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                s.set_nodelay(nagle).expect("Nagle algorithm is removed");
                handle_connection(s);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                wait_for_fd();
                continue;
            }
            Err(e) => panic!("encountered IO error:{e}"),
        }
    }
    println!("Jai Ballaya");
}

fn handle_connection(mut stream: TcpStream) {
    println!("Connection Recieved");
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let response = "HTTP/1.1 200 OK\r\n\r\nHello from Non-Blocking Rust!";
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => {
            println!("Failedto read from stream:{}", e)
        }
    }
}

fn wait_for_fd() {
    thread::sleep(Duration::from_millis(10));
}
