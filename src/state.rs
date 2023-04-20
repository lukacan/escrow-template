use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]

pub enum JanecekState {
    Party {
        is_initialized: bool,
        author: Pubkey,
        voting_state: Pubkey,
        created: i64,
        name: String,
        votes: i64,
        bump: u8,
    },
    Voter {
        is_initialized: bool,
        author: Pubkey,
        voting_state: Pubkey,
        num_votes: VotesStates,
        pos1: Pubkey,
        pos2: Pubkey,
        neg1: Pubkey,
        bump: u8,
    },
    VotingState {
        is_initialized: bool,
        voting_owner: Pubkey,
        voting_started: i64,
        voting_ends: i64,
        bump: u8,
    },
    VotingOwner {
        is_initialized: bool,
        author: Pubkey,
        voting_state: Pubkey,
        bump: u8,
    },
}
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum VotesStates {
    NoMoreVotes,
    NoMorePositiveVotes,
    OneSpent,
    Full,
}
impl JanecekState {
    pub const NAME_LENGTH: usize = 32;
    pub const LEN_PARTY: usize = 1 + 1 + 32 + 32 + 8 + 4 + JanecekState::NAME_LENGTH * 4 + 8 + 1;
    pub const LEN_VOTER: usize = 1 + 1 + 32 + 32 + 1 + 32 + 32 + 32 + 1;
    pub const LEN_VOTINGSTATE: usize = 1 + 1 + 32 + 8 + 8 + 1;
    pub const LEN_VOTINGOWNER: usize = 1 + 1 + 32 + 32 + 1;
}
