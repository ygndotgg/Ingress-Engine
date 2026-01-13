use std::time::Duration;

use tokio::{net::TcpStream, time::sleep};
use Ingress_Engine::config::Config;

#[tokio::main]
pub async fn main() {
    let target = "127.0.0.1:1883";
    let connection_count = Config::global().max_connections;
    let mut handles = vec![];
    println!("Starting the Swarm Attack on {}", target);
    for i in 1..=connection_count {
        let handle = tokio::spawn(async move {
            match TcpStream::connect(target).await {
                Ok(_stream) => {
                    println!("Client #{} connected! (Holding Line...)", i);
                    loop {
                        sleep(Duration::from_secs(3600)).await;
                    }
                }
                Err(e) => {
                    println!("Client #{}: REJECTED ({})", i, e);
                }
            }
        });
        handles.push(handle);
        sleep(Duration::from_millis(50)).await;
    }
    sleep(Duration::from_secs(60)).await;
}
