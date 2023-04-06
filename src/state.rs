use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct Party{
    pub is_initialized: bool,
    pub author: Pubkey,
    pub created: i64,
    pub voting_ends: i64,
    pub name: String,
    pub votes: i64,
}

pub struct Voter{
    pub is_initialized: bool,
    pub author: Pubkey,
    pub num_votes: i8,
    pub pos1: Pubkey,
    pub pos2: Pubkey,
    pub neg1: Pubkey,
}


impl Sealed for Party {}

impl IsInitialized for Party {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Party{
    const LEN: usize = 32 // author
    + 8 // created
    + 8 // voting ends
    + 4 // vector prefix
    + 32 * 4 // number of bytes * size of char
    + 8 // votes
    + 1; // is_initialzed
    
    
    
    fn unpack_from_slice(src: &[u8]) -> Result<Self, solana_program::program_error::ProgramError> {
        Ok(())
    }


    fn pack_into_slice(&self, dst: &mut [u8]) {
    }

}


impl Sealed for Voter{}

impl IsInitialized for Voter{
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Voter{
    const LEN: usize = 32 //author
    + 1 // num votes
    + 32 // pos1
    + 32 // pos2
    + 32 // neg1
    + 1; // is_initialzed

    fn unpack_from_slice(src: &[u8]) -> Result<Self, solana_program::program_error::ProgramError> {
        Ok(())
    }


    fn pack_into_slice(&self, dst: &mut [u8]) {

    }
}

