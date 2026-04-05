use pinocchio::{AccountView, Address, ProgramResult, entrypoint, error::ProgramError};
use pinocchio::sysvars::Sysvar;
use pinocchio::sysvars::rent::Rent;

entrypoint!(process_instruction, 0);

fn process_instruction(
    _program_id: &Address,
    _accounts: &mut [AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    let space = 100;
    let rent = Rent::get()?;
    let lamports = rent.try_minimum_balance(space)?;
    if lamports == 1_586_880 {
        Ok(())
    } else {
        Err(ProgramError::Custom(1))
    }
}
