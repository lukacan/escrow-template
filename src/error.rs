// inside error.rs
use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq)]
pub enum JanecekError {
    // trying to use as many errors as possible from ProgramError
    // pubkeys mismatch
    #[error("Voting owner ID mismatch")]
    VotingOwnerMismatch,
    #[error("Voting state ID mismatch")]
    VotingStateMismatch,
    #[error("Voter Mismatch")]
    VoterMismatch,

    // misc
    #[error("Addition overflow")]
    AdditionOverflow,
    #[error("Subtraction overflow")]
    SubtractionOverflow,
    #[error("Incorrect account type")]
    DiscriminantMismatch,

    // context accounts
    #[error("Account is not mutable")]
    AccountNotmutable,
    #[error("Read-only account required")]
    AccountMutable,

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
impl From<JanecekError> for u64 {
    fn from(value: JanecekError) -> Self {
        value as u64
    }
}
