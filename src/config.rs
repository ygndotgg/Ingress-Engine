use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::sync::OnceLock;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub max_connections: usize,
    pub tcp_recv_buf_size: usize,
    pub tcp_nodelay: bool,
}

pub static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn default() -> Self {
        Self {
            max_connections: 1000,
            tcp_recv_buf_size: 4096,
            tcp_nodelay: true,
        }
    }
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
    pub fn global() -> &'static Config {
        GLOBAL_CONFIG.get().expect("Config is not initialized")
    }
}
