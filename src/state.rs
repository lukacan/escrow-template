use crate::error::JanecekError;
use solana_program::{
    program_error::ProgramError,
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};



pub mod state {
    const NAME_LENGTH: usize = 32;

    use solana_program::{program_pack::{Sealed, Pack, IsInitialized}};

    use super::*;
    #[derive(Debug)]
    pub struct Party {
        pub is_initialized: bool,
        pub author: Pubkey,
        pub created: i64,
        pub voting_ends: i64,
        pub name: String,
        pub votes: i64,
        pub bump:u8,
    }

    pub struct Voter {
        pub is_initialized: bool,
        pub author: Pubkey,
        pub num_votes: i8,
        pub pos1: Pubkey,
        pub pos2: Pubkey,
        pub neg1: Pubkey,
    }

    impl IsInitialized for Party {
        fn is_initialized(&self) -> bool {
            self.is_initialized
        }
    }

    impl Sealed for Party{}

    impl Pack for Party{
        const LEN: usize = 1 // is_initialzed
        + 32 // author
        + 8 // created
        + 8 // voting ends
        + 4 // vector prefix
        + NAME_LENGTH * 4 // number of bytes * size of char
        + 8 // votes
        + 1; // bump

        fn pack_into_slice(&self, dst: &mut [u8]) {
            let dst = array_mut_ref![dst, 0, Party::LEN];
            
            let (
                is_initialized_dst,
                author_dst,
                created_dst,
                voting_ends_dst,
                name_len_dst,
                name_dst,
                votes_dst,
                bump_dst,
            ) = mut_array_refs![dst, 1, 32, 8, 8, 4,4*NAME_LENGTH,8,1];
    
            let Party {
                is_initialized,
                author,
                created,
                voting_ends,
                name,
                votes,
                bump,
            } = self;

            // check if name is too long
            let name_len = self.name.chars().count() as u32;
            is_initialized_dst[0] = *is_initialized as u8;
            author_dst.copy_from_slice(author.as_ref());
            *created_dst = created.to_le_bytes();
            *voting_ends_dst = voting_ends.to_le_bytes();
            *name_len_dst = name_len.to_le_bytes();
            name_dst[..name_len as usize].copy_from_slice(name.as_bytes());
            *votes_dst = votes.to_be_bytes();
            bump_dst[0] = *bump;



        }
        fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
            let src = array_ref![src, 0, Party::LEN];
            let (
                is_initialized,
                author,
                created,
                voting_ends,
                name_len,
                name,
                votes,
                bump,
            ) = array_refs![src, 1, 32, 8, 8, 4,4*NAME_LENGTH,8,1];



            let name_len_ = u32::from_le_bytes(*name_len);
            
            if name_len_ > 32{
                return Err(JanecekError::StringTooLong.into());
            }


            let is_initialized = match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            };

            let bump = bump[0];
    
            Ok(Party {
                is_initialized,
                author: Pubkey::new_from_array(*author),
                created: i64::from_le_bytes(*created),
                voting_ends: i64::from_le_bytes(*voting_ends),
                name: <String as borsh::BorshDeserialize>::try_from_slice(&name[0..name_len_ as usize]).unwrap(),
                votes:i64::from_le_bytes(*votes),
                bump:bump,
            })
        }
    }
}
