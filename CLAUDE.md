# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

BTC Toolkit is a Rust GUI application for managing Bitcoin ASIC mining farms, built with the Iced framework. It provides real-time network scanning, miner discovery, and farm management capabilities through a professional Bitcoin-themed interface.

## Development Commands

```bash
# Quick compilation check
cargo check

# Run the application
cargo run

# Build optimized release
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Architecture Overview

### Message-Driven State Management

The application uses Iced's message-driven architecture with a **hierarchical state management pattern**:

- `BtcToolkit` in `main.rs` acts as the central coordinator, routing messages between components
- Each UI component (Dashboard, NetworkConfig, ScanningView) manages local state but shares a central `AppConfig`
- Cross-component navigation and state changes flow through the main coordinator

Key pattern: Components don't directly change application state - they send messages that bubble up to the coordinator, which then triggers cascading updates across affected components.

### Dual-Runtime Scanner Architecture

The network scanner implements a sophisticated **dual-runtime pattern** to integrate async network operations with Iced's immediate-mode GUI:

1. **Iced Runtime**: Manages UI events and subscriptions
2. **Tokio Runtime**: Created in separate threads for network scanning
3. **Channel Bridge**: `tokio::sync::mpsc::unbounded_channel` connects the runtimes

Scanner messages flow through Iced's subscription system (`Scanner::scan_multiple_groups()`) enabling real-time streaming of miner discoveries to the UI.

### Configuration State Synchronization

The system maintains two distinct state layers:

- **Persistent State** (`AppConfig`): Serialized to `btc_toolkit_config.json`, shared immutably across components
- **Transient UI State**: Component-specific editing state, progress indicators, UI toggles

Complex pattern in `NetworkConfig`: Maintains shadow editing state (`editing_group: Option<EditingGroup>`) that doesn't affect main config until explicitly saved, enabling cancel functionality.

### Streaming Scanner Integration

The most architecturally complex interaction is the "Scanning Session" lifecycle:

1. **Initiation**: Dashboard → Main coordinator → Creates ScanningView + sets `active_scan` state
2. **Execution**: Subscription system → Scanner → Real-time `ScannerMessage`s → ScanningView updates
3. **Completion**: ScanningView → Main coordinator → Merges results into `AppConfig` → Dashboard refresh → Disk persistence

### Network Module Utilities

Shared utilities in `src/network/mod.rs` eliminate code duplication:
- `create_miner_factory(network_range)` - Basic factory from network range
- `create_configured_miner_factory(network_range, config)` - Factory with filters applied  
- `estimate_ip_count(network_range)` - IP count estimation

## Key Dependencies

- **Iced v0.13** - Cross-platform GUI framework with reactive architecture
- **asic-rs** - ASIC miner communication library (git dependency from 256-Foundation)
- **Tokio v1.47** - Async runtime with full multi-threading features
- **MiMalloc** - High-performance secure memory allocator
- **Serde** - JSON serialization for configuration persistence

## Configuration Management

Application state persists to `btc_toolkit_config.json` containing:
- Scan groups with network ranges (CIDR or IP range notation)
- Miner filtering configuration (manufacturer and firmware filters)
- Last scan results and group enable/disable states

The config uses automatic fallback creation if the file doesn't exist or fails to load.

## Build Optimizations

Release builds are optimized for performance:
- LTO (Link Time Optimization) enabled
- Symbol stripping for smaller binaries
- Single codegen unit for maximum optimization
- Secure memory allocator for enhanced security