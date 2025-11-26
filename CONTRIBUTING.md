# Contributing to Sflowana

Thank you for your interest in contributing to Sflowana.

Sflowana is an early-stage, performance-focused Solana infrastructure project. The project prioritizes correctness, clear architecture, predictable behavior, and measurable performance.

## Development

Make sure you have a current stable Rust toolchain installed.

Check the project:

```bash
cargo check --all-targets --all-features
```

Run tests:

```bash
cargo test --all-targets --all-features
```

Format the code:

```bash
cargo fmt --all
```

Run Clippy:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

All of the above should pass before submitting a pull request.

## Architecture

Keep the architecture small and explicit.

In particular:

* Keep Solana ingestion separate from protocol-specific decoding.
* Keep protocol-specific logic isolated from the core event pipeline.
* Prefer bounded concurrency and explicit backpressure.
* Avoid unnecessary allocations, serialization, and data copying in hot paths.
* Prefer simple implementations before introducing specialized optimizations.
* Introduce abstractions when they solve an actual problem.
* Avoid adding dependencies without a clear reason.

Sflowana is intended to process real-time blockchain data, so correctness must not be sacrificed for superficial performance improvements.

## Protocol Decoders

Protocol-specific changes should include appropriate test data whenever possible.

A decoder should:

* Correctly identify supported instructions.
* Reject or safely handle malformed input.
* Produce deterministic normalized events.
* Avoid panicking on untrusted blockchain data.

Unknown or unsupported instructions should not normally terminate the processing pipeline.

## Tests

New functionality should include appropriate tests.

Decoder changes should include representative transaction or instruction fixtures when practical.

Concurrency-sensitive functionality should include tests for the relevant concurrent behavior.

Regression tests should be added when fixing a previously discovered bug.

## Performance

Performance is an important project goal.

Performance-sensitive changes should be supported by benchmarks or measurements when practical.

Do not make performance claims based solely on intuition.

Prefer:

```text
measure
   ↓
identify bottleneck
   ↓
change
   ↓
measure again
```

Report relevant changes to:

* Throughput
* Latency
* CPU usage
* Memory usage
* Allocation behavior

When benchmarking networked components, distinguish external network latency from Sflowana's internal processing latency.

## Commits

Use clear, conventional commit messages.

Examples:

```text
feat(ingestion): add rpc transaction source

feat(decoder): decode raydium swap instructions

fix(decoder): handle malformed instruction data

test(decoder): add raydium swap fixtures

bench(decoder): measure swap decoding throughput

docs(architecture): document event pipeline

refactor(ingestion): simplify reconnect handling
```

Keep commits focused on one coherent change.

## Pull Requests

Pull requests should explain:

* What changed
* Why it changed
* How it was tested
* Any relevant performance impact
* Important architectural trade-offs

For protocol changes, include the relevant protocol behavior or transaction examples used to validate the implementation.

Avoid combining unrelated refactoring with functional changes.

## Architecture Decisions

Significant architectural decisions should be documented as ADRs under:

```text
docs/adr/
```

An ADR is appropriate when a decision has meaningful consequences for:

* System architecture
* Data flow
* Concurrency
* Performance
* External dependencies
* Protocol integration
* Persistence or state management

Do not create ADRs for trivial implementation details.

## Security

Do not report security vulnerabilities through public GitHub issues.

See [SECURITY.md](SECURITY.md) for the security reporting policy.

## Scope

Sflowana is intentionally focused on real-time Solana data infrastructure.

Contributions that add unrelated functionality, unnecessary abstractions, or substantial complexity without a clear use case may not fit the project's direction.

The goal is not to maximize the number of features.

The goal is to build a small, correct, measurable, high-performance system.
