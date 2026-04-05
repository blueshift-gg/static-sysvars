# Benchmarks

## Interpreter

Benchmark results averaged over 100,000 iterations

##### BASELINE (Reference)

| Method | Total (ms) | Avg (µs) | CUs |
|--------|-----------|----------|-----|
| Baseline (empty program) | 4532.47 | 45.32 | 2 |

##### SYSVAR ACCESS METHODS

| Method | Total (ms) | Avg (µs) | CUs | Overhead (µs) | vs Fastest |
|--------|-----------|----------|-----|---------------|------------|
| Static Sysvars | 4609.28 | 46.09 | 7 | 0.768 | 1.00x |
| Syscall | 4758.85 | 47.59 | 125 | 2.264 | 2.95x |
| Sysvar Account (unchecked) | 5031.37 | 50.31 | 9 | 4.989 | 6.50x |
| Sysvar Account (checked) | 5255.03 | 52.55 | 27 | 7.226 | 9.41x |

## JIT (x86)

Benchmark results averaged over 100,000 iterations

##### BASELINE (Reference)

| Method | Total (ms) | Avg (µs) | CUs |
|--------|-----------|----------|-----|
| Baseline (empty program) | 3304.04 | 33.04 | 2 |

##### SYSVAR ACCESS METHODS

| Method | Total (ms) | Avg (µs) | CUs | Overhead (µs) | vs Fastest |
|--------|-----------|----------|-----|---------------|------------|
| Static Sysvars | 3336.75 | 33.37 | 7 | 0.327 | 1.00x |
| Syscall | 3399.53 | 33.99 | 125 | 0.955 | 2.92x |
| Sysvar Account (unchecked) | 3600.72 | 36.01 | 9 | 2.967 | 9.07x |
| Sysvar Account (checked) | 3959.75 | 39.60 | 27 | 6.557 | 20.05x |

Note: CUs = Compute Units per iteration
      Overhead = Method Time - Baseline (isolates sysvar access cost)
      vs Fastest = Method Overhead / Fastest Overhead (1.0x = fastest method)
test bench_all ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 17.61s

     Running tests/tests.rs (target/debug/deps/tests-9e338049580ff0bf)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

## JIT (Arm64 Mac with Rosetta)

Benchmark results averaged over 100,000 iterations

##### BASELINE (Reference)

| Method | Total (ms) | Avg (µs) | CUs |
|--------|-----------|----------|-----|
| Baseline (empty program) | 7965.38 | 79.65 | 2 |

##### SYSVAR ACCESS METHODS

| Method | Total (ms) | Avg (µs) | CUs | Overhead (µs) | vs Fastest |
|--------|-----------|----------|-----|---------------|------------|
| Static Sysvars | 8186.84 | 81.87 | 7 | 2.215 | 1.00x |
| Syscall | 8454.93 | 84.55 | 125 | 4.896 | 2.21x |
| Sysvar Account (unchecked) | 9555.16 | 95.55 | 9 | 15.898 | 7.18x |
| Sysvar Account (checked) | 10917.65 | 109.18 | 27 | 29.523 | 13.33x |
