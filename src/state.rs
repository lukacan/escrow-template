use crate::error::JanecekError;
use solana_program::{
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    system_program,
    sysvar::Sysvar,
    rent::Rent,
    program::invoke_signed
};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;


pub mod state {
    const name_length: usize = 32;

    use super::*;
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

    pub struct CreateParty<'info>{
        pub author: AccountInfo<'info>, 
        pub party: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
    }

    impl Party {
        const LEN: usize = 1 // is_initialzed
        + 32 // author
        + 8 // created
        + 8 // voting ends
        + 4 // vector prefix
        + name_length * 4 // number of bytes * size of char
        + 8; // votes
    }
    impl<'info> CreateParty<'info>
    {
        pub fn try_accounts(
            program_id: &Pubkey,
            accounts: & mut &[AccountInfo<'info>],
            ix_data: &[u8],
            bumps: &mut std::collections::BTreeMap<String, u8>,
        ) -> Result<Self,ProgramError>
        {
            let mut ix_data = ix_data;

            /// create struct for instruction data
            #[derive(BorshDeserialize,BorshSerialize)]
            struct Args {
                name: String,
            }

            // deserialized instruction data
            let Args {name} = Args::deserialize(&mut ix_data).map_err(|_| {
                JanecekError::InstructionDidNotDeserialize
            })?;


            // check if accounts are empty, if not , read AccountInfo and expect it as signer
            if accounts.is_empty(){
                return Err(JanecekError::AccountNotEnoughKeys.into());
            }

            // first account, we expect it as signer
            let author: &AccountInfo = &accounts[0];
            *accounts = &accounts[1..];

            if !author.is_signer{
                return Err(JanecekError::AccountNotSigner.into());
            }


            if accounts.is_empty(){
                return Err(JanecekError::AccountNotEnoughKeys.into());
            }

            // read next account and expect party
            // this part is tricky, because we need to check pda
            let party: &AccountInfo = &accounts[0];
            *accounts = &accounts[1..];

            if accounts.is_empty(){
                return Err(JanecekError::AccountNotEnoughKeys.into());
            }

            let system_program: &AccountInfo = &accounts[0];
            *accounts = &accounts[1..];

            if *system_program.owner != system_program::ID{
                return Err(JanecekError::AccountNotSystemOwned.into());
            }

            let (pda,bump) = Pubkey::find_program_address(
                &[name.as_bytes()], 
                program_id,);
            
            
            bumps.insert("party".to_string(), bump);

            let rent = Rent::get()?;


            let party = {
                let actual_owner = party.owner;
                let space = Party::LEN;
                let pa = if !false || actual_owner == &system_program::ID
                {
                    let payer = author;
                    let current_lamports = party.lamports();
                    if current_lamports == 0{
                        let lamports_needed = rent.minimum_balance(space);
                        let instr = system_instruction::create_account(
                            payer.key,
                            party.key,
                            lamports_needed,
                            space as u64,
                            program_id,
                        );

                        invoke_signed(
                            &instr, 
                            &[payer.clone(),party.clone()],
                            &[
                                &[name.as_bytes()],
                                &[&[bump]]
                                ])?;
                    }
                    else {
                        let required_lamports = rent.
                        minimum_balance(space)
                        .max(1)
                        .saturating_sub(current_lamports);
                        
                        if required_lamports > 0{
                            let instr = system_instruction::transfer(
                                payer.key,
                                party.key, 
                                required_lamports);

                                invoke_signed(
                                    &instr, 
                                    &[payer.clone(),party.clone()],
                                    &[
                                        &[name.as_bytes()],
                                        &[&[bump]]
                                        ])?;
                        }

                        let instr = system_instruction::allocate(
                            party.key, 
                            space as u64);
                        
                        
                        invoke_signed(
                            &instr,
                            &[party.clone()], 
                            &[
                                &[name.as_bytes()],
                                &[&[bump]]
                                ])?;

                        let instr = system_instruction::assign(
                            party.key, 
                            program_id);

                            
                        invoke_signed(
                                &instr,
                                &[party.clone()], 
                                &[
                                    &[name.as_bytes()],
                                    &[&[bump]]
                                    ])?;
                            
                    }
                    party

                }
                else{
                    party
                };
                pa
            };

            if *party.key != pda{
                return Err(JanecekError::PdaMismatch.into());
            }
            if !party.is_writable{
                return Err(JanecekError::ConstraintMut.into());
            }
            if (!rent
                .is_exempt(
                    party.lamports(), 
                    party.try_data_len()?)){
                        return Err(JanecekError::ConstraintRentExempt.into());  

            }
            if !author.is_writable{
                return Err(JanecekError::ConstraintMut.into());
            }

            Ok(CreateParty { 
                author: author.clone(),
                party: party.clone(),
                system_program: system_program.clone(),
            })
        }
    }
}
