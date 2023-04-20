use crate::processor;
use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};
declare_id!("Fnambs3f1XXoMmAVc94bf8t6JDAxmVkXz85XU4v2edph");
entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    processor::entry(program_id, accounts, instruction_data)
}
