// inside error.rs
use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum JanecekError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not rent exempt")]
    NotRentExempt,
    #[error("Program ID Mismatch")]
    ProgramIDMismatch,
    #[error("Instruction Missmatch")]
    MissmatchInstruction,
    #[error("Instruction Fallback not Found")]
    InstructionFallbackNotFound,
    #[error("String too Long")]
    StringTooLong,
    #[error("Instruction Did Not Deserialize")]
    InstructionDidNotDeserialize,
    #[error("Not enough account keys")]
    AccountNotEnoughKeys,
    #[error("Account is not signer")]
    AccountNotSigner,
    #[error("System program ID mismatch")]
    AccountNotSystemOwned,
    #[error("PDA mismatch from provided address")]
    PdaMismatch,
    #[error("Constrain mutable")]
    ConstraintMut,
    #[error("Rent Exempt")]
    ConstraintRentExempt,

}

impl From<JanecekError> for ProgramError {
    fn from(e: JanecekError) -> Self {
        ProgramError::Custom(e as u32)
    }
}