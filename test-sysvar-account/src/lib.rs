use pinocchio::{AccountView, Address, ProgramResult, entrypoint, error::ProgramError};

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
    let lamports = unsafe { *(rent.data_ptr() as *const u64) } * (space + 128);
    if lamports == 1_586_880 {
        Ok(())
    } else {
        Err(ProgramError::Custom(1))
    }
}
