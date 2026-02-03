# Ingress-Engine

A high-performance, asynchronous MQTT broker implementation written in Rust. Ingress-Engine is designed for efficient message brokering with support for concurrent client connections and configurable network settings.

## Features

- **Asynchronous Networking**: Built on Tokio for non-blocking I/O and high concurrency
- **MQTT Protocol Support**: Full MQTT broker implementation
- **Configurable**: Easy-to-customize settings via TOML configuration
- **High Performance**: Optimized for handling thousands of concurrent connections
- **Logging**: Integrated logging via `env_logger`

## Requirements

- Rust 1.70 or later (edition 2021)
- Cargo

## Installation

Clone the repository:

```bash
git clone <repository-url>
cd Ingress-Engine
```

Build the project:

```bash
cargo build --release
```

## Configuration

Configuration is managed through `config.toml`. Key settings include:

- `max_connections`: Maximum number of concurrent client connections (default: 1024)
- `tcp_recv_buf_size`: Size of TCP receive buffer in bytes (default: 4096)
- `tcp_nodelay`: Disable Nagle's algorithm for lower latency (default: true)

## Usage

### Running the Server

```bash
cargo run --release
```

The server will start on the default address `0.0.0.0:1883` (standard MQTT port).

### Custom Bind Address

Set the `BIND_ADDR` environment variable to use a different address:

```bash
BIND_ADDR=127.0.0.1:9999 cargo run
```

### Logging

Control logging level with the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run
```

## Project Structure

- `src/main.rs` - Application entry point
- `src/lib.rs` - Library module definitions
- `src/network.rs` - Network connector and broker implementation
- `src/server.rs` - Server logic
- `src/connection.rs` - Connection handling
- `src/async_conn.rs` - Asynchronous connection utilities
- `src/config.rs` - Configuration management
- `examples/attack.rs` - Example or test client

## Dependencies

- `tokio` - Asynchronous runtime
- `serde` - Serialization framework
- `toml` - TOML parsing for configuration
- `log` & `env_logger` - Logging infrastructure
- `bytes` - Efficient byte handling
- `thiserror` - Error handling utilities

## Examples

An example attack/test client is provided in `examples/attack.rs`.

## License

[Specify your license here]

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.
