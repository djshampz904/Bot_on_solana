use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    acccount_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pukey,
};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct HelloWorldInstruction {
    pub message: String,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = HelloWorldInstruction::try_from_slice(instruction_data)
        .expect("Failed to deserialize instruction");

    msg!("Hello, {}", instruction.message);

    Ok(())
}