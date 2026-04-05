use pinocchio::{AccountView, Address, ProgramResult, entrypoint, error::ProgramError};
use pinocchio::sysvars::rent::Rent;

entrypoint!(process_instruction, 1);

fn process_instruction(
    _program_id: &Address,
    accounts: &mut [AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    let space = 100;
    let [rent] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let lamports = Rent::from_account_view(rent)?.try_minimum_balance(space)?;
    if lamports == 1_586_880 {
        Ok(())
    } else {
        Err(ProgramError::Custom(1))
    }
}
