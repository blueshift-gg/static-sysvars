use mollusk_svm::{Mollusk, result::Check};
use pinocchio::Address;
use solana_instruction::AccountMeta;
use std::time::Instant;

const PROGRAM_ID: [u8; 32] = [0x02; 32];
const ITERATIONS: u32 = 100_000;

fn suppress_logs() {
    std::env::set_var("RUST_LOG", "off");
}

struct BenchResult {
    name: &'static str,
    total_ns: u128,
    avg_ns: u128,
    compute_units: u64,
}

#[test]
fn bench_all() {
    suppress_logs();
    let mut results = Vec::new();

    // Benchmark: Baseline
    {
        let mut mollusk = Mollusk::new(&PROGRAM_ID.into(), "target/deploy/test_baseline");
        mollusk.logger = None;
        mollusk.feature_set.deactivate(&Address::from_str_const("ssysSnr9pRX3YF6adCcu4RaerCCtZNqjkymQpqD7yhP"));

        let instruction = solana_instruction::Instruction {
            program_id: PROGRAM_ID.into(),
            accounts: vec![],
            data: vec![],
        };

        let checks: &[Check] = &[Check::success()];
        let chain: Vec<(&solana_instruction::Instruction, &[Check])> =
            vec![(&instruction, checks); ITERATIONS as usize];

        let start = Instant::now();
        let result = mollusk.process_and_validate_instruction_chain(&chain, &[]);
        let elapsed = start.elapsed();

        results.push(BenchResult {
            name: "Baseline (empty program)",
            total_ns: elapsed.as_nanos(),
            avg_ns: elapsed.as_nanos() / ITERATIONS as u128,
            compute_units: result.compute_units_consumed / ITERATIONS as u64,
        });
    }

    // Benchmark: Static Sysvars
    {
        let mut mollusk = Mollusk::new(&PROGRAM_ID.into(), "target/deploy/test_static_sysvars");
        mollusk.logger = None;

        let instruction = solana_instruction::Instruction {
            program_id: PROGRAM_ID.into(),
            accounts: vec![],
            data: vec![],
        };

        let checks: &[Check] = &[Check::success()];
        let chain: Vec<(&solana_instruction::Instruction, &[Check])> =
            vec![(&instruction, checks); ITERATIONS as usize];

        let start = Instant::now();
        let result = mollusk.process_and_validate_instruction_chain(&chain, &[]);
        let elapsed = start.elapsed();

        results.push(BenchResult {
            name: "Static Sysvars",
            total_ns: elapsed.as_nanos(),
            avg_ns: elapsed.as_nanos() / ITERATIONS as u128,
            compute_units: result.compute_units_consumed / ITERATIONS as u64,
        });
    }

    // Benchmark: Syscall
    {
        let mut mollusk = Mollusk::new(&PROGRAM_ID.into(), "target/deploy/test_syscall");
        mollusk.feature_set.deactivate(&Address::from_str_const("ssysSnr9pRX3YF6adCcu4RaerCCtZNqjkymQpqD7yhP"));
        mollusk.logger = None;

        let instruction = solana_instruction::Instruction {
            program_id: PROGRAM_ID.into(),
            accounts: vec![],
            data: vec![],
        };

        let checks: &[Check] = &[Check::success()];
        let chain: Vec<(&solana_instruction::Instruction, &[Check])> =
            vec![(&instruction, checks); ITERATIONS as usize];

        let start = Instant::now();
        let result = mollusk.process_and_validate_instruction_chain(&chain, &[]);
        let elapsed = start.elapsed();

        results.push(BenchResult {
            name: "Syscall",
            total_ns: elapsed.as_nanos(),
            avg_ns: elapsed.as_nanos() / ITERATIONS as u128,
            compute_units: result.compute_units_consumed / ITERATIONS as u64,
        });
    }

    // Benchmark: Sysvar Account (unchecked)
    {
        let mut mollusk = Mollusk::new(&PROGRAM_ID.into(), "target/deploy/test_sysvar_account");
        mollusk.feature_set.deactivate(&Address::from_str_const("ssysSnr9pRX3YF6adCcu4RaerCCtZNqjkymQpqD7yhP"));
        mollusk.logger = None;

        let (rent, rent_account) = mollusk.sysvars.keyed_account_for_rent_sysvar();

        let instruction = solana_instruction::Instruction {
            program_id: PROGRAM_ID.into(),
            accounts: vec![AccountMeta::new_readonly(rent, false)],
            data: vec![],
        };

        let accounts = &[(rent, rent_account)];
        let checks: &[Check] = &[Check::success()];
        let chain: Vec<(&solana_instruction::Instruction, &[Check])> =
            vec![(&instruction, checks); ITERATIONS as usize];

        let start = Instant::now();
        let result = mollusk.process_and_validate_instruction_chain(&chain, accounts);
        let elapsed = start.elapsed();

        results.push(BenchResult {
            name: "Sysvar Account (unchecked)",
            total_ns: elapsed.as_nanos(),
            avg_ns: elapsed.as_nanos() / ITERATIONS as u128,
            compute_units: result.compute_units_consumed / ITERATIONS as u64,
        });
    }

    // Benchmark: Sysvar Account (checked)
    {
        let mut mollusk = Mollusk::new(&PROGRAM_ID.into(), "target/deploy/test_sysvar_account_checked");
        mollusk.feature_set.deactivate(&Address::from_str_const("ssysSnr9pRX3YF6adCcu4RaerCCtZNqjkymQpqD7yhP"));
        mollusk.logger = None;

        let (rent, rent_account) = mollusk.sysvars.keyed_account_for_rent_sysvar();

        let instruction = solana_instruction::Instruction {
            program_id: PROGRAM_ID.into(),
            accounts: vec![AccountMeta::new_readonly(rent, false)],
            data: vec![],
        };

        let accounts = &[(rent, rent_account)];
        let checks: &[Check] = &[Check::success()];
        let chain: Vec<(&solana_instruction::Instruction, &[Check])> =
            vec![(&instruction, checks); ITERATIONS as usize];

        let start = Instant::now();
        let result = mollusk.process_and_validate_instruction_chain(&chain, accounts);
        let elapsed = start.elapsed();

        results.push(BenchResult {
            name: "Sysvar Account (checked)",
            total_ns: elapsed.as_nanos(),
            avg_ns: elapsed.as_nanos() / ITERATIONS as u128,
            compute_units: result.compute_units_consumed / ITERATIONS as u64,
        });
    }

    // Print results table
    println!("\n{}", "=".repeat(110));
    println!("BENCHMARK RESULTS ({} iterations)", ITERATIONS);
    println!("{}", "=".repeat(110));

    let baseline_avg = results[0].avg_ns;

    // Show baseline separately
    println!("\nBASELINE (Reference):");
    println!("{:<35} {:>15} {:>15} {:>10}", "Method", "Total (ms)", "Avg (µs)", "CUs");
    println!("{}", "-".repeat(75));
    let total_ms = results[0].total_ns as f64 / 1_000_000.0;
    let avg_us = results[0].avg_ns as f64 / 1_000.0;
    println!("{:<35} {:>15.2} {:>15.2} {:>10}", results[0].name, total_ms, avg_us, results[0].compute_units);

    // Find fastest non-baseline implementation
    let fastest_overhead = results[1..]
        .iter()
        .map(|r| r.avg_ns.saturating_sub(baseline_avg))
        .min()
        .unwrap_or(0);

    // Show sysvar implementations with regression against fastest
    println!("\n\nSYSVAR ACCESS METHODS:");
    println!("{:<35} {:>15} {:>15} {:>10} {:>18} {:>18}", "Method", "Total (ms)", "Avg (µs)", "CUs", "Overhead (µs)", "vs Fastest");
    println!("{}", "-".repeat(120));

    for result in &results[1..] {
        let total_ms = result.total_ns as f64 / 1_000_000.0;
        let avg_us = result.avg_ns as f64 / 1_000.0;
        let overhead_ns = result.avg_ns.saturating_sub(baseline_avg);
        let overhead_us = overhead_ns as f64 / 1_000.0;
        let regression = overhead_ns as f64 / fastest_overhead as f64;

        println!("{:<35} {:>15.2} {:>15.2} {:>10} {:>18.3} {:>18.2}x",
            result.name, total_ms, avg_us, result.compute_units, overhead_us, regression);
    }

    println!("{}", "=".repeat(120));
    println!("\nNote: CUs = Compute Units per iteration");
    println!("      Overhead = Method Time - Baseline (isolates sysvar access cost)");
    println!("      vs Fastest = Method Overhead / Fastest Overhead (1.0x = fastest method)");
}
