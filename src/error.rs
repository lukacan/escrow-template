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
    InstructionFallbackNotFound
}

impl From<JanecekError> for ProgramError {
    fn from(e: JanecekError) -> Self {
        ProgramError::Custom(e as u32)
    }
}