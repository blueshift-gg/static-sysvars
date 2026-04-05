use pinocchio::{AccountView, Address, ProgramResult, entrypoint};

entrypoint!(process_instruction, 0);

fn process_instruction(
    _program_id: &Address,
    _accounts: &mut [AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}
