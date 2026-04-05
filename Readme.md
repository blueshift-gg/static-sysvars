# Static Sysvars Benchmarks

Performance comparison of different sysvar access methods on Solana.

## Quick Start

Build all test programs:
```sh
cargo build-sbf --workspace
```

Run correctness test:
```sh
cargo test-sbf test_ --workspace
```

Run benchmarks:
```sh
cargo test bench_all -- --nocapture
```

Run benchmarks with JIT (x86_64 Mac only, uses Rosetta):
```sh
cargo test bench_all --target=x86_64-apple-darwin -- --nocapture
```

## How It Works

Static sysvars use deterministic memory addresses computed from sysvar names:

1. Hash the sysvar name: `murmur3("SOL_RENT_SYSVAR") = 0x494df715`
2. Map to static region: `(hash & 0xffffffff) | (0x05 << 32) = 0x5_494df715`
3. VM intercepts reads at these addresses and returns data directly from the SysvarCache

This eliminates syscall overhead and the need to pass sysvar accounts as instruction parameters.

## Dependencies

Uses custom forks with static sysvar support:
- **solana-sbpf** — Memory mapping interception for static sysvar region
- **agave** — SysvarCache registration in program runtime
- **mollusk-svm** — Test harness with static sysvar feature flag