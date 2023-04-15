// inside error.rs
use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq)]
pub enum JanecekError {
    // instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Instruction Did Not Deserialize")]
    InstructionDidNotDeserialize,

    // pubkeys mismatch
    #[error("Program ID Mismatch")]
    ProgramIDMismatch,
    #[error("System ID mysmatch")]
    SystemIDMismatch,
    #[error("PDA mismatch")]
    PdaMismatch,
    #[error("Voting owner ID mismatch")]
    VotingOwnerMismatch,
    #[error("Voting state ID mismatch")]
    VotingStateMismatch,
    #[error("Account owner mismatch")]
    AccountOwnerMismatch,
    #[error("Voter Mismatch")]
    VoterMismatch,

    // misc
    #[error("Addition overflow")]
    AdditionOverflow,
    #[error("Subtraction overflow")]
    SubtractionOverflow,
    #[error("String too Long")]
    StringTooLong,
    #[error("Incorrect account type")]
    DiscriminantMismatch,

    // context accounts
    #[error("Account is not signer")]
    AccountNotSigner,
    #[error("Account is not mutable")]
    AccountNotmutable,
    #[error("Read-only account required")]
    AccountMutable,
    #[error("Rent Exempt")]
    ConstraintRentExempt,
    #[error("Account is already initialized")]
    AccountAlreadyInitialized,
    #[error("Account not Initialzed yet")]
    AccountNotInitialized,

    // votes
    #[error("Can`t vote 2 times for same Party")]
    NoBothPosSameParty,
    #[error("Number of votes out of range")]
    VotesOutOfRange,
    #[error("No more positive votes")]
    NoMorePosVotes,
    #[error("No more positive votes")]
    NoMoreVotes,
    #[error("Before voting negative, vote 2 times positive")]
    VoteNegativeConstrain,
    #[error("Voting Ended")]
    VotingEnded,
}

impl From<JanecekError> for ProgramError {
    fn from(e: JanecekError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
