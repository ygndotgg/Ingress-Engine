use std::env;

use Ingress_Engine::{
    config::{Config, GLOBAL_CONFIG},
    network::NetworkConnector,
    server::Conf,
};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_file("config.toml").unwrap_or_else(|_| {
        eprintln!("Config file not found using the default");
        Config::default()
    });
    if GLOBAL_CONFIG.set(config).is_err() {
        eprintln!("Config has already been set!");
    }

    env_logger::init();
    let addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:1883".to_string());
    let reactor = NetworkConnector::new(&addr).await;
    reactor.run().await;
    Ok(())
}
