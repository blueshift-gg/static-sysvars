use mollusk_svm::{Mollusk, result::Check};
use pinocchio::Address;
use solana_instruction::AccountMeta;

const PROGRAM_ID: [u8; 32] = [0x02; 32];

#[test]
fn test_static_sysvar() {
    let mollusk = Mollusk::new(&PROGRAM_ID.into(), "target/deploy/test_static_sysvars");
    
    let instruction = solana_instruction::Instruction {
        program_id: PROGRAM_ID.into(),
        accounts: vec![],
        data: vec![],
    };

    mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
}

#[test]
fn test_sysvar_account() {
    let mut mollusk = Mollusk::new(&PROGRAM_ID.into(), "target/deploy/test_sysvar_account");

    mollusk.sysvars.rent.lamports_per_byte = 6960;

    let (rent, rent_account) = mollusk.sysvars.keyed_account_for_rent_sysvar();

    let instruction = solana_instruction::Instruction {
        program_id: PROGRAM_ID.into(),
        accounts: vec![
            AccountMeta::new_readonly(rent, false),
        ],
        data: vec![],
    };

    mollusk.process_and_validate_instruction(&instruction, &[
        (rent, rent_account),
    ], &[Check::success()]);
}

#[test]
fn test_syscall() {
    let mut mollusk = Mollusk::new(&PROGRAM_ID.into(), "target/deploy/test_syscall");

    mollusk.feature_set.deactivate(&Address::from_str_const("ssysSnr9pRX3YF6adCcu4RaerCCtZNqjkymQpqD7yhP"));

    let instruction = solana_instruction::Instruction {
        program_id: PROGRAM_ID.into(),
        accounts: vec![],
        data: vec![],
    };

    mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
}

#[test]
fn test_baseline() {
    let mut mollusk = Mollusk::new(&PROGRAM_ID.into(), "target/deploy/test_baseline");

    mollusk.feature_set.deactivate(&Address::from_str_const("ssysSnr9pRX3YF6adCcu4RaerCCtZNqjkymQpqD7yhP"));

    let instruction = solana_instruction::Instruction {
        program_id: PROGRAM_ID.into(),
        accounts: vec![],
        data: vec![],
    };

    mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
}