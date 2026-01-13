use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

pub async fn async_conn() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            socket.nodelay().unwrap();
            let mut buf = [0; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(s) if s == 0 => return,
                    Ok(s) => s,
                    Err(_) => return,
                };
                socket.write_all(&buf[0..n]).await.unwrap();
            }
        });
    }
}
