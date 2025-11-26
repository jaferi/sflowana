# Sflowana

**High-performance real-time Solana transaction and market-data engine written in Rust.**

Sflowana ingests Solana activity, decodes protocol interactions, normalizes domain events, and streams structured data to latency-sensitive downstream systems.

It is designed as infrastructure for systems such as:

* Trading systems
* MEV/searcher infrastructure
* Market-data services
* Blockchain analytics
* Protocol monitoring
* Real-time Solana applications

Sflowana is **not** a trading bot, DEX, wallet, or trading strategy.

Its purpose is to provide the real-time data layer underneath those systems.

## Architecture

The intended processing pipeline is:

```text
Solana
   │
   ▼
Ingestion
   │
   ▼
Transaction Processing
   │
   ▼
Protocol Decoders
   │
   ▼
Event Normalization
   │
   ├──────────────► Hot State
   │
   ▼
Event Streaming
   │
   ├── Trading
   ├── MEV
   ├── Analytics
   └── Monitoring
```

The initial implementation will start with Solana RPC-based ingestion and progressively introduce streaming sources, protocol-specific decoders, replay, and performance benchmarking.

## Goals

Sflowana is being built to explore and demonstrate:

* Rust systems programming
* Async and concurrent architecture
* Solana transaction processing
* Protocol decoding
* Real-time event pipelines
* Low-latency networking
* Backpressure and bounded concurrency
* WebSocket streaming
* Deterministic replay
* Performance measurement

## Non-Goals

Sflowana will not become a general-purpose:

* DEX
* Trading bot
* Wallet
* MEV strategy
* Block explorer
* RPC node
* Analytics dashboard

Those systems may consume Sflowana's output, but they are outside its core scope.

## Development

Check the project:

```bash
cargo check
```

Run tests:

```bash
cargo test
```

Run formatting:

```bash
cargo fmt --all
```

Run linting:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Run the application:

```bash
cargo run
```

## Project Status

Sflowana is currently in the initial architecture and implementation stage.

The first milestone is a clean Solana ingestion and transaction-processing pipeline. Protocol-specific market-event decoding and real-time streaming will be introduced incrementally.

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

## Security

For information about reporting security vulnerabilities, see [SECURITY.md](SECURITY.md).

## License

Sflowana is licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.
