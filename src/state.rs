use crate::error::JanecekError;
use solana_program::{
    program_error::ProgramError,
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};



pub mod state {
    const name_length: usize = 32;

    use solana_program::program_pack::{Sealed, Pack, IsInitialized};

    use super::*;
    #[derive(Debug)]
    pub struct Party {
        pub is_initialized: bool,
        pub author: Pubkey,
        pub created: i64,
        pub voting_ends: i64,
        pub name: String,
        pub votes: i64,
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
        + name_length * 4 // number of bytes * size of char
        + 8; // votes

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
            ) = mut_array_refs![dst, 1, 32, 8, 8, 4,4*name_length,8];
    
            let Party {
                is_initialized,
                author,
                created,
                voting_ends,
                name,
                votes,
            } = self;
    
            let name_len = name.len() as u32;
            is_initialized_dst[0] = *is_initialized as u8;
            author_dst.copy_from_slice(author.as_ref());
            *created_dst = created.to_le_bytes();
            *voting_ends_dst = voting_ends.to_le_bytes();
            *name_len_dst = name_len.to_le_bytes();
            name_dst.copy_from_slice(name.as_bytes());
            *votes_dst = votes.to_be_bytes();



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
            ) = array_refs![src, 1, 32, 8, 8, 4,4*name_length,8];


            let name_len_ = u32::from_le_bytes(*name_len);

            if name_len_ > 32{

            }


            let is_initialized = match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            };
    
            Ok(Party {
                is_initialized,
                author: Pubkey::new_from_array(*author),
                created: i64::from_le_bytes(*created),
                voting_ends: i64::from_le_bytes(*voting_ends),
                name: String::from_utf8(name.to_vec()).unwrap(),
                votes:i64::from_le_bytes(*votes),
            })
        }
    }

    // impl<'info> CreateParty<'info>
    // {
    //     pub fn try_accounts(
    //         program_id: &Pubkey,
    //         accounts: & mut &[AccountInfo<'info>],
    //         ix_data: &[u8],
    //         bumps: &mut std::collections::BTreeMap<String, u8>,
    //     ) -> Result<Self,ProgramError>
    //     {
    //         let mut ix_data = ix_data;

    //         /// create struct for instruction data
    //         #[derive(BorshDeserialize,BorshSerialize)]
    //         struct Args {
    //             name: String,
    //         }

    //         // deserialized instruction data
    //         let Args {name} = Args::deserialize(&mut ix_data).map_err(|_| {
    //             JanecekError::InstructionDidNotDeserialize
    //         })?;


    //         // check if accounts are empty, if not , read AccountInfo and expect it as signer
    //         if accounts.is_empty(){
    //             return Err(JanecekError::AccountNotEnoughKeys.into());
    //         }

    //         // first account, we expect it as signer
    //         let author: &AccountInfo = &accounts[0];
    //         *accounts = &accounts[1..];

    //         if !author.is_signer{
    //             return Err(JanecekError::AccountNotSigner.into());
    //         }


    //         if accounts.is_empty(){
    //             return Err(JanecekError::AccountNotEnoughKeys.into());
    //         }

    //         // read next account and expect party
    //         // this part is tricky, because we need to check pda
    //         let party: &AccountInfo = &accounts[0];
    //         *accounts = &accounts[1..];

    //         if accounts.is_empty(){
    //             return Err(JanecekError::AccountNotEnoughKeys.into());
    //         }

    //         let system_program: &AccountInfo = &accounts[0];
    //         *accounts = &accounts[1..];

    //         if *system_program.owner != system_program::ID{
    //             return Err(JanecekError::AccountNotSystemOwned.into());
    //         }

    //         let (pda,bump) = Pubkey::find_program_address(
    //             &[name.as_bytes()], 
    //             program_id,);
            
            
    //         bumps.insert("party".to_string(), bump);

    //         let rent = Rent::get()?;


    //         let party = {
    //             let actual_owner = party.owner;
    //             let space = Party::LEN;
    //             let pa = if !false || actual_owner == &system_program::ID
    //             {
    //                 let payer = author;
    //                 let current_lamports = party.lamports();
    //                 if current_lamports == 0{
    //                     let lamports_needed = rent.minimum_balance(space);
    //                     let instr = system_instruction::create_account(
    //                         payer.key,
    //                         party.key,
    //                         lamports_needed,
    //                         space as u64,
    //                         program_id,
    //                     );

    //                     invoke_signed(
    //                         &instr, 
    //                         &[payer.clone(),party.clone()],
    //                         &[
    //                             &[name.as_bytes()],
    //                             &[&[bump]]
    //                             ])?;
    //                 }
    //                 else {
    //                     let required_lamports = rent.
    //                     minimum_balance(space)
    //                     .max(1)
    //                     .saturating_sub(current_lamports);
                        
    //                     if required_lamports > 0{
    //                         let instr = system_instruction::transfer(
    //                             payer.key,
    //                             party.key, 
    //                             required_lamports);

    //                             invoke_signed(
    //                                 &instr, 
    //                                 &[payer.clone(),party.clone()],
    //                                 &[
    //                                     &[name.as_bytes()],
    //                                     &[&[bump]]
    //                                     ])?;
    //                     }

    //                     let instr = system_instruction::allocate(
    //                         party.key, 
    //                         space as u64);
                        
                        
    //                     invoke_signed(
    //                         &instr,
    //                         &[party.clone()], 
    //                         &[
    //                             &[name.as_bytes()],
    //                             &[&[bump]]
    //                             ])?;

    //                     let instr = system_instruction::assign(
    //                         party.key, 
    //                         program_id);

                            
    //                     invoke_signed(
    //                             &instr,
    //                             &[party.clone()], 
    //                             &[
    //                                 &[name.as_bytes()],
    //                                 &[&[bump]]
    //                                 ])?;
                            
    //                 }
    //                 party

    //             }
    //             else{
    //                 party
    //             };
    //             pa
    //         };

    //         if *party.key != pda{
    //             return Err(JanecekError::PdaMismatch.into());
    //         }
    //         if !party.is_writable{
    //             return Err(JanecekError::ConstraintMut.into());
    //         }
    //         if (!rent
    //             .is_exempt(
    //                 party.lamports(), 
    //                 party.try_data_len()?)){
    //                     return Err(JanecekError::ConstraintRentExempt.into());  

    //         }
    //         if !author.is_writable{
    //             return Err(JanecekError::ConstraintMut.into());
    //         }

    //         Ok(CreateParty { 
    //             author: author.clone(),
    //             party: party.clone(),
    //             system_program: system_program.clone(),
    //         })
    //     }
    // }
}
