use crate::error::JanecekError;
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub mod state {
    const NAME_LENGTH: usize = 32;

    use solana_program::program_pack::{IsInitialized, Pack, Sealed};

    use super::*;
    #[derive(Debug)]
    pub struct Party {
        pub is_initialized: bool,
        pub author: Pubkey,
        pub voting_state: Pubkey,
        pub created: i64,
        pub name: String,
        pub votes: i64,
        pub bump: u8,
    }

    impl IsInitialized for Party {
        fn is_initialized(&self) -> bool {
            self.is_initialized
        }
    }

    impl Sealed for Party {}

    impl Pack for Party {
        const LEN: usize = 1    // is_initialzed
        + 32                    // author
        + 32                    // voting state
        + 8                     // created
        + 4                     // vector prefix
        + NAME_LENGTH * 4       // number of bytes * size of char
        + 8                     // votes
        + 1; // bump

        fn pack_into_slice(&self, dst: &mut [u8]) {
            let dst = array_mut_ref![dst, 0, Party::LEN];

            let (
                is_initialized_dst,
                author_dst,
                voting_state_dst,
                created_dst,
                name_len_dst,
                name_dst,
                votes_dst,
                bump_dst,
            ) = mut_array_refs![dst, 1, 32, 32, 8, 4, 4 * NAME_LENGTH, 8, 1];

            let Party {
                is_initialized,
                author,
                voting_state,
                created,
                name,
                votes,
                bump,
            } = self;

            // check if name is too long
            let name_len = name.chars().count() as u32;
            is_initialized_dst[0] = *is_initialized as u8;
            author_dst.copy_from_slice(author.as_ref());
            voting_state_dst.copy_from_slice(voting_state.as_ref());
            *created_dst = created.to_le_bytes();
            *name_len_dst = name_len.to_le_bytes();
            name_dst[..name_len as usize].copy_from_slice(name.as_bytes());
            *votes_dst = votes.to_be_bytes();
            bump_dst[0] = *bump;
        }
        fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
            let src = array_ref![src, 0, Party::LEN];
            let (is_initialized, author, voting_state, created, name_len, name, votes, bump) =
                array_refs![src, 1, 32, 32, 8, 4, 4 * NAME_LENGTH, 8, 1];

            let name_len_ = u32::from_le_bytes(*name_len);

            if name_len_ > 32 {
                return Err(JanecekError::StringTooLong.into());
            }

            let is_initialized = match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            };

            let tmp_name = String::from_utf8(name[0..name_len_ as usize].to_vec());

            let name = match tmp_name {
                Ok(_) => tmp_name.unwrap(),
                Err(_) => String::from(""),
            };
            let bump = bump[0];

            Ok(Party {
                is_initialized,
                author: Pubkey::new_from_array(*author),
                voting_state: Pubkey::new_from_array(*voting_state),
                created: i64::from_le_bytes(*created),
                name: name,
                votes: i64::from_le_bytes(*votes),
                bump: bump,
            })
        }
    }

    pub struct Voter {
        pub is_initialized: bool,
        pub author: Pubkey,
        pub voting_state: Pubkey,
        pub num_votes: u8,
        pub pos1: Pubkey,
        pub pos2: Pubkey,
        pub neg1: Pubkey,
        pub bump: u8,
    }

    impl IsInitialized for Voter {
        fn is_initialized(&self) -> bool {
            self.is_initialized
        }
    }

    impl Sealed for Voter {}
    impl Pack for Voter {
        const LEN: usize = 1    // is_initialzed
        + 32                    // author
        + 32                    // voting state
        + 1                     // num votes
        + 32                    // pos1
        + 32                    // pos2
        + 32                    // neg1
        + 1; // bump

        fn pack_into_slice(&self, dst: &mut [u8]) {
            let dst = array_mut_ref![dst, 0, Voter::LEN];

            let (
                is_initialized_dst,
                author_dst,
                voting_state_dst,
                num_votes_dst,
                pos1_dst,
                pos2_dst,
                neg1_dst,
                bump_dst,
            ) = mut_array_refs![dst, 1, 32, 32, 1, 32, 32, 32, 1];

            let Voter {
                is_initialized,
                author,
                voting_state,
                num_votes,
                pos1,
                pos2,
                neg1,
                bump,
            } = self;

            is_initialized_dst[0] = *is_initialized as u8;
            author_dst.copy_from_slice(author.as_ref());
            voting_state_dst.copy_from_slice(voting_state.as_ref());
            num_votes_dst[0] = *num_votes;
            pos1_dst.copy_from_slice(pos1.as_ref());
            pos2_dst.copy_from_slice(pos2.as_ref());
            neg1_dst.copy_from_slice(neg1.as_ref());
            bump_dst[0] = *bump;
        }

        fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
            let src = array_ref![src, 0, Voter::LEN];
            let (is_initialized, author, voting_state, num_votes, pos1, pos2, neg1, bump) =
                array_refs![src, 1, 32, 32, 1, 32, 32, 32, 1];

            let is_initialized = match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            };

            let bump = bump[0];
            let num_votes = num_votes[0];

            Ok(Voter {
                is_initialized,
                author: Pubkey::new_from_array(*author),
                voting_state: Pubkey::new_from_array(*voting_state),
                num_votes: num_votes,
                pos1: Pubkey::new_from_array(*pos1),
                pos2: Pubkey::new_from_array(*pos2),
                neg1: Pubkey::new_from_array(*neg1),
                bump: bump,
            })
        }
    }

    pub struct VotingState {
        pub is_initialized: bool,
        pub voting_owner: Pubkey,
        pub voting_started: i64,
        pub voting_ends: i64,
        pub bump: u8,
    }

    impl IsInitialized for VotingState {
        fn is_initialized(&self) -> bool {
            self.is_initialized
        }
    }

    impl Sealed for VotingState {}
    impl Pack for VotingState {
        const LEN: usize = 1    // is initialzed
        + 32                    // voting owner
        + 8                     // voting started
        +8                      // voting ends
        +1; // bump

        fn pack_into_slice(&self, dst: &mut [u8]) {
            let dst = array_mut_ref![dst, 0, VotingState::LEN];

            let (
                is_initialized_dst,
                voting_owner_dst,
                voting_started_dst,
                voting_ends_dst,
                bump_dst,
            ) = mut_array_refs![dst, 1, 32, 8, 8, 1];

            let VotingState {
                is_initialized,
                voting_owner,
                voting_started,
                voting_ends,
                bump,
            } = self;

            is_initialized_dst[0] = *is_initialized as u8;
            voting_owner_dst.copy_from_slice(voting_owner.as_ref());
            *voting_started_dst = voting_started.to_le_bytes();
            *voting_ends_dst = voting_ends.to_le_bytes();
            bump_dst[0] = *bump;
        }

        fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
            let src = array_ref![src, 0, VotingState::LEN];
            let (is_initialized, voting_owner, voting_started, voting_ends, bump) =
                array_refs![src, 1, 32, 8, 8, 1];

            let is_initialized = match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            };

            let bump = bump[0];

            Ok(VotingState {
                is_initialized,
                voting_owner: Pubkey::new_from_array(*voting_owner),
                voting_started: i64::from_le_bytes(*voting_started),
                voting_ends: i64::from_le_bytes(*voting_ends),
                bump: bump,
            })
        }
    }

    pub struct VotingOwner {
        pub is_initialized: bool,
        pub owner: Pubkey,
        pub voting_state: Pubkey,
        pub bump: u8,
    }
    impl IsInitialized for VotingOwner {
        fn is_initialized(&self) -> bool {
            self.is_initialized
        }
    }

    impl Sealed for VotingOwner {}
    impl Pack for VotingOwner {
        const LEN: usize = 1    // is initialzed
        + 32                    // initializer
        + 32                    // voting state
        +1; // bump

        fn pack_into_slice(&self, dst: &mut [u8]) {
            let dst = array_mut_ref![dst, 0, VotingOwner::LEN];

            let (is_initialized_dst, initializer_dst, voting_state_dst, bump_dst) =
                mut_array_refs![dst, 1, 32, 32, 1];

            let VotingOwner {
                is_initialized,
                owner,
                voting_state,
                bump,
            } = self;

            is_initialized_dst[0] = *is_initialized as u8;
            initializer_dst.copy_from_slice(owner.as_ref());
            voting_state_dst.copy_from_slice(voting_state.as_ref());
            bump_dst[0] = *bump;
        }

        fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
            let src = array_ref![src, 0, VotingOwner::LEN];
            let (is_initialized, owner, voting_state, bump) = array_refs![src, 1, 32, 32, 1];

            let is_initialized = match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            };

            let bump = bump[0];

            Ok(VotingOwner {
                is_initialized,
                owner: Pubkey::new_from_array(*owner),
                voting_state: Pubkey::new_from_array(*voting_state),
                bump: bump,
            })
        }
    }
}
