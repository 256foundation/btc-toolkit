# BTC Toolkit

A GUI tool for scanning and managing BTC ASIC miners on local networks.

## About

BTC Toolkit provides an intuitive interface for discovering and interacting with Bitcoin mining hardware. Similar to BTCTools but built with modern Rust technologies:

- **GUI**: Built with [iced.rs](https://github.com/iced-rs/iced) (v0.13)
- **Hardware Communication**: Leverages [asic-rs](https://github.com/b-rowan/asic-rs) (Rust rewrite of pyasic)
- **Performance**: Uses mimalloc for efficient memory allocation

## Status

🚧 **Early Development** - Basic UI framework implemented

## Usage

### Requirements

- Rust toolchain (stable channel)

### Running

```bash
cargo run
```

### Building

```bash
cargo build --release
```

## Planned Features

- Network scanning for ASIC miners
- Hardware information display
- Batch configuration
- Performance monitoring
