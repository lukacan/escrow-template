use crate::error::JanecekError;
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub mod state {
    pub const NAME_LENGTH: usize = 32;

    use solana_program::program_pack::{IsInitialized, Pack, Sealed};

    use super::*;

    pub const VOTINGOWNER: u8 = 31;
    pub const VOTINGSTATE: u8 = 32;
    pub const PARTY: u8 = 33;
    pub const VOTER: u8 = 34;

    pub struct Party {
        pub discriminant: u8,
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
        const LEN: usize = 1    // discriminant
        + 1                     // is_initialzed
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
                discriminant_dst,
                is_initialized_dst,
                author_dst,
                voting_state_dst,
                created_dst,
                name_len_dst,
                name_dst,
                votes_dst,
                bump_dst,
            ) = mut_array_refs![dst, 1, 1, 32, 32, 8, 4, 4 * NAME_LENGTH, 8, 1];

            let Party {
                discriminant,
                is_initialized,
                author,
                voting_state,
                created,
                name,
                votes,
                bump,
            } = self;

            let name_len:u32 = u32::try_from(name.chars().count()).unwrap();
            discriminant_dst[0] = u8::try_from(*discriminant).unwrap();
            is_initialized_dst[0] = u8::try_from(*is_initialized).unwrap();
            author_dst.copy_from_slice(author.as_ref());
            voting_state_dst.copy_from_slice(voting_state.as_ref());
            *created_dst = created.to_le_bytes();
            *name_len_dst = name_len.to_le_bytes();
            name_dst[.. usize::try_from(name_len).unwrap()].copy_from_slice(name.as_bytes());
            *votes_dst = votes.to_be_bytes();
            bump_dst[0] = u8::try_from(*bump).unwrap();
        }
        fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
            let src = array_ref![src, 0, Party::LEN];
            let (
                discriminant,
                is_initialized,
                author,
                voting_state,
                created,
                name_len,
                name,
                votes,
                bump,
            ) = array_refs![src, 1, 1, 32, 32, 8, 4, 4 * NAME_LENGTH, 8, 1];

            let bump = u8::try_from(bump[0]).unwrap();
            let discriminant = u8::try_from(discriminant[0]).unwrap();
            let name_len_ = u32::try_from(u32::from_le_bytes(*name_len)).unwrap();

            let is_initialized = match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            };

            if is_initialized && name_len_ > 32 {
                return Err(ProgramError::InvalidInstructionData);
            }
            if is_initialized && discriminant != PARTY {
                return Err(JanecekError::DiscriminantMismatch.into());
            }

            let tmp_name = String::from_utf8(name[0..usize::try_from(name_len_).unwrap()].to_vec());

            let name = match tmp_name {
                Ok(_) => tmp_name.unwrap(),
                Err(_) => String::from(""),
            };

            Ok(Party {
                discriminant,
                is_initialized,
                author: Pubkey::new_from_array(*author),
                voting_state: Pubkey::new_from_array(*voting_state),
                created: i64::try_from(i64::from_le_bytes(*created)).unwrap(),
                name: name,
                votes: i64::try_from(i64::from_le_bytes(*votes)).unwrap(),
                bump: bump,
            })
        }
    }

    pub struct Voter {
        pub discriminant: u8,
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
        const LEN: usize = 1    // discriminant
        + 1                     // is_initialzed
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
                discriminant_dst,
                is_initialized_dst,
                author_dst,
                voting_state_dst,
                num_votes_dst,
                pos1_dst,
                pos2_dst,
                neg1_dst,
                bump_dst,
            ) = mut_array_refs![dst, 1, 1, 32, 32, 1, 32, 32, 32, 1];

            let Voter {
                discriminant,
                is_initialized,
                author,
                voting_state,
                num_votes,
                pos1,
                pos2,
                neg1,
                bump,
            } = self;

            discriminant_dst[0] = u8::try_from(*discriminant).unwrap();
            is_initialized_dst[0] = u8::try_from(*is_initialized).unwrap();
            author_dst.copy_from_slice(author.as_ref());
            voting_state_dst.copy_from_slice(voting_state.as_ref());
            num_votes_dst[0] = u8::try_from(*num_votes).unwrap();
            pos1_dst.copy_from_slice(pos1.as_ref());
            pos2_dst.copy_from_slice(pos2.as_ref());
            neg1_dst.copy_from_slice(neg1.as_ref());
            bump_dst[0] = u8::try_from(*bump).unwrap();
        }

        fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
            let src = array_ref![src, 0, Voter::LEN];
            let (
                discriminant,
                is_initialized,
                author,
                voting_state,
                num_votes,
                pos1,
                pos2,
                neg1,
                bump,
            ) = array_refs![src, 1, 1, 32, 32, 1, 32, 32, 32, 1];

            let is_initialized = match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            };

            let bump = u8::try_from(bump[0]).unwrap();
            let num_votes = u8::try_from(num_votes[0]).unwrap();
            let discriminant = u8::try_from(discriminant[0]).unwrap();

            if is_initialized && discriminant != VOTER {
                return Err(JanecekError::DiscriminantMismatch.into());
            }

            Ok(Voter {
                discriminant,
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
        pub discriminant: u8,
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
        const LEN: usize = 1    // discriminant
        + 1                     // is initialzed
        + 32                    // voting owner
        + 8                     // voting started
        +8                      // voting ends
        +1; // bump

        fn pack_into_slice(&self, dst: &mut [u8]) {
            let dst = array_mut_ref![dst, 0, VotingState::LEN];

            let (
                discriminant_dst,
                is_initialized_dst,
                voting_owner_dst,
                voting_started_dst,
                voting_ends_dst,
                bump_dst,
            ) = mut_array_refs![dst, 1, 1, 32, 8, 8, 1];

            let VotingState {
                discriminant,
                is_initialized,
                voting_owner,
                voting_started,
                voting_ends,
                bump,
            } = self;

            discriminant_dst[0] = u8::try_from(*discriminant).unwrap();
            is_initialized_dst[0] = u8::try_from(*is_initialized).unwrap();
            voting_owner_dst.copy_from_slice(voting_owner.as_ref());
            *voting_started_dst = voting_started.to_le_bytes();
            *voting_ends_dst = voting_ends.to_le_bytes();
            bump_dst[0] = u8::try_from(*bump).unwrap();
        }

        fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
            let src = array_ref![src, 0, VotingState::LEN];
            let (discriminant, is_initialized, voting_owner, voting_started, voting_ends, bump) =
                array_refs![src, 1, 1, 32, 8, 8, 1];

            let is_initialized = match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            };

            let bump = u8::try_from(bump[0]).unwrap();
            let discriminant = u8::try_from(discriminant[0]).unwrap();

            if is_initialized && discriminant != VOTINGSTATE {
                return Err(JanecekError::DiscriminantMismatch.into());
            }

            Ok(VotingState {
                discriminant,
                is_initialized,
                voting_owner: Pubkey::new_from_array(*voting_owner),
                voting_started: i64::try_from(i64::from_le_bytes(*voting_started)).unwrap(),
                voting_ends: i64::try_from(i64::from_le_bytes(*voting_ends)).unwrap(),
                bump: bump,
            })
        }
    }

    pub struct VotingOwner {
        pub discriminant: u8,
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
        const LEN: usize = 1    // discriminant
        + 1                     // is initialzed
        + 32                    // initializer
        + 32                    // voting state
        +1; // bump

        fn pack_into_slice(&self, dst: &mut [u8]) {
            let dst = array_mut_ref![dst, 0, VotingOwner::LEN];

            let (discriminant_dst, is_initialized_dst, initializer_dst, voting_state_dst, bump_dst) =
                mut_array_refs![dst, 1, 1, 32, 32, 1];

            let VotingOwner {
                discriminant,
                is_initialized,
                owner,
                voting_state,
                bump,
            } = self;

            discriminant_dst[0] = u8::try_from(*discriminant).unwrap();
            is_initialized_dst[0] = u8::try_from(*is_initialized).unwrap();
            initializer_dst.copy_from_slice(owner.as_ref());
            voting_state_dst.copy_from_slice(voting_state.as_ref());
            bump_dst[0] = u8::try_from(*bump).unwrap();
        }

        fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
            let src = array_ref![src, 0, VotingOwner::LEN];
            let (discriminant, is_initialized, owner, voting_state, bump) =
                array_refs![src, 1, 1, 32, 32, 1];

            let is_initialized = match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            };

            let bump = u8::try_from(bump[0]).unwrap();
            let discriminant = u8::try_from(discriminant[0]).unwrap();

            
            if is_initialized && discriminant != VOTINGOWNER {
                return Err(JanecekError::DiscriminantMismatch.into());
            }
            Ok(VotingOwner {
                discriminant,
                is_initialized,
                owner: Pubkey::new_from_array(*owner),
                voting_state: Pubkey::new_from_array(*voting_state),
                bump: bump,
            })
        }
    }
}
