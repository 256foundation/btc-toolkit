# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Context7 Usage

Always use Context7 MCP tools (`resolve-library-id` then `get-library-docs`) when you need code generation, setup/configuration steps, or library/API documentation. Do this automatically without being asked.

## Build Commands

```bash
cargo run              # Run in debug mode
cargo build --release  # Build optimized release binary
cargo check            # Fast compilation check
cargo test             # Run tests
cargo clippy           # Lint
```

Cross-compilation (requires appropriate toolchains):

```bash
cargo build --release --target x86_64-unknown-linux-gnu    # Linux
cargo build --release --target x86_64-pc-windows-gnu       # Windows (needs gcc-mingw-w64 when cross compiling from linux)
```

## Architecture

**BTC Toolkit** is a desktop GUI for managing Bitcoin ASIC mining farms, built with iced.rs (Elm architecture) and asic-rs for miner communication.

### Core Pattern: Elm Architecture

The app uses iced's message-passing architecture:

- **State** (`BtcToolkit` in main.rs) - Application data
- **Messages** (`BtcToolkitMessage`) - Events that modify state
- **Update** (`update()`) - Processes messages, returns `Task<Message>`
- **View** (`view()`) - Renders state as widgets

### Module Structure

- `main.rs` - App bootstrap, page routing, message dispatch
- `main_view.rs` - Dashboard with miner list and scan controls
- `device_detail_view.rs` - Individual miner detail page
- `network_config.rs` - Scan group configuration UI
- `network/scanner.rs` - Async network scanner using iced subscriptions
- `network/full_fetch.rs` - Full miner data fetcher
- `config.rs` - JSON config persistence (`btc_toolkit_config.json`)
- `health.rs` - Miner health assessment (chips, hashrate, temp, fans)
- `theme/` - Design system (colors, typography, icons, containers)

### Key Dependencies

- **iced 0.14** - GUI framework with `svg` and `tokio` features
- **asic-rs** - Miner communication (supports Bitmain, MicroBT, Canaan, BitAxe, etc.)
- **mimalloc** - High-performance allocator

### Async Pattern

The app runs on iced's internal tokio runtime (via `tokio` feature). Do NOT use `#[tokio::main]` - this causes nested runtime panics. Use:

- `Task::perform()` for one-shot async operations
- `Subscription::run_with()` for ongoing streams (like network scanning)

### Subscription Pattern (iced 0.14)

For subscriptions with data, use boxed streams:

```rust
fn my_subscription(data: MyData) -> Subscription<Message> {
    Subscription::run_with(data, |data| {
        stream::channel(100, |output| async move { ... }).boxed()
    })
}
```

Data types must implement `Hash` - use JSON serialization for complex types.

## Code Style

Write concise, pragmatic, maintainable, idiomatic, modern, type-safe, secure, performant, and production-ready Rust code.
