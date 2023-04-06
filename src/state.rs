

use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey
};


pub struct Party{
    pub author: Pubkey,
    pub created: i64,
    pub voting_ends: i64,
    pub name: String,
    pub votes: i64,
    pub bump: u8,
}

impl Party{
    const LEN: usize = 32 // author
    + 8 // created
    + 8 // voting ends
    + 4 // vector prefix
    + 32 * 4 // number of bytes * size of char
    + 8; // votes
}

