use std::mem::size_of;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
#[repr(u8)]
pub enum JanecekState {
    // "" If a discriminant for a variant is not specified,
    // then it is set to one higher than the discriminant
    // of the previous variant in the declaration. ""
    // https://doc.rust-lang.org/reference/items/enumerations.html
    // consider during initialization, all values are set to 0, so
    // let`s have discriminants from 1
    Fresh,
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
    pub const LEN_PARTY: usize = size_of::<u8>()
        + size_of::<bool>()
        + size_of::<Pubkey>()
        + size_of::<Pubkey>()
        + size_of::<i64>()
        + size_of::<u32>()
        + JanecekState::NAME_LENGTH * 4
        + size_of::<i64>()
        + size_of::<u8>();
    pub const LEN_VOTER: usize = size_of::<u8>()
        + size_of::<bool>()
        + size_of::<Pubkey>()
        + size_of::<Pubkey>()
        + 1
        + size_of::<Pubkey>()
        + size_of::<Pubkey>()
        + size_of::<Pubkey>()
        + size_of::<u8>();
    pub const LEN_VOTINGSTATE: usize = size_of::<u8>()
        + size_of::<bool>()
        + size_of::<Pubkey>()
        + size_of::<i64>()
        + size_of::<i64>()
        + size_of::<u8>();
    pub const LEN_VOTINGOWNER: usize = size_of::<u8>()
        + size_of::<bool>()
        + size_of::<Pubkey>()
        + size_of::<Pubkey>()
        + size_of::<u8>();
}
