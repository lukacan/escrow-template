// inside error.rs
use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum JanecekError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
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
    #[error("Account is already initialized")]
    AccountAlreadyInitialized,
    #[error("Vote positive first")]
    VotePosFirst,
    #[error("Can`t vote 2 times for same Party")]
    NoBothPosSameParty,
    #[error("Number of votes out of range")]
    VotesOutOfRange,
    #[error("Account not Initialzed yet")]
    AccountNotInitialized,
    #[error("Addition overflow")]
    AdditionOverflow,
    #[error("Subtraction overflow")]
    SubtractionOverflow,
    #[error("Voting Ended")]
    VotingEnded,
    #[error("Voting owner mismatch")]
    VotingOwnerMismatch,
    #[error("Voting state mismatch")]
    VotingStateMismatch,
}

impl From<JanecekError> for ProgramError {
    fn from(e: JanecekError) -> Self {
        ProgramError::Custom(e as u32)
    }
}