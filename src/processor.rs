use std::result;

use borsh::BorshDeserialize;
use solana_program::account_info::next_account_info;

use solana_program::{program, system_program};
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
};

use crate::identifier::ID;
use crate::state::state::Party;
use crate::{error::JanecekError, instruction::instruction};

pub struct Processor;
impl Processor {
    pub fn entry(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        Self::try_entry(program_id, accounts, instruction_data).map_err(|e| {
            //e.log();
            e.into()
        })
    }
    fn try_entry(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
        // check if program ID is correct
        if *program_id != ID {
            return Err(JanecekError::ProgramIDMismatch.into());
        }

        // check if data contains at least 1 byte, so function can be decoded
        if data.len() < 1 {
            return Err(JanecekError::MissmatchInstruction.into());
        }
        Self::dispatch(program_id, accounts, data)
    }

    fn dispatch(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
        let mut ix_data: &[u8] = data;

        let sighash: [u8; 1] = {
            let mut sighash: [u8; 1] = [0; 1];
            sighash.copy_from_slice(&ix_data[..1]);
            ix_data = &ix_data[1..];
            sighash
        };

        match sighash {
            [0] => {
                msg!("Instruction: CreateParty");
                Self::process_create_party(program_id, accounts, ix_data)
            }
            [1] => {
                msg!("Instruction: CreateVoter");
                Self::process_create_voter(program_id, accounts, ix_data)
            }
            [2] => {
                msg!("Instruction: VotePositive");
                Self::process_create_party(program_id, accounts, ix_data)
            }
            [3] => {
                msg!("Instruction: VoteNegative");
                Self::process_create_party(program_id, accounts, ix_data)
            }
            _ => Err(JanecekError::InstructionFallbackNotFound.into()),
        }
    }

    fn process_create_party(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ix_data: &[u8],
    ) -> ProgramResult {
        let ix = instruction::CreateParty::deserialize(&mut &ix_data[..])
            .map_err(|_| JanecekError::InstructionDidNotDeserialize)?;

        msg!("Instruction data Deserialized");
        let instruction::CreateParty { 
            name ,
            bump} = ix;

        let accounts_iter = &mut accounts.iter();
        
        let author = next_account_info(accounts_iter)?;
        msg!("Author Loaded");


        if !author.is_signer {
            return Err(JanecekError::AccountNotSigner.into());
        }

        let party = next_account_info(accounts_iter)?;
        msg!("Party Loaded");

        assert!(author.is_writable);
        assert!(author.is_signer);
        assert!(party.is_writable);
        assert_eq!(party.owner, &system_program::ID);

        //let system_program = next_account_info(accounts_iter)?;

        //assert!(current_program.is_writable);

        //assert!(system_program::check_id(system_program.key));

        msg!("System Program Loaded");


        let (pda_on, bump_on) = Pubkey::find_program_address(
            &[name.as_bytes()], 
            program_id);

        assert_eq!(party.key, &pda_on);

        msg!("PDA Address Created");
        let current_lamports = party.lamports();

        let rent = Rent::get()?;
        if current_lamports == 0 {
            let lamports_needed = rent.minimum_balance(Party::LEN);

            let instr = system_instruction::create_account(
                author.key,
                party.key,
                lamports_needed,
                Party::LEN as u64,
                program_id,
            );

            msg!("Invoking Signed");
            msg!("Executing here");
            msg!("{}",author.key);
            msg!("{}",program_id);
            msg!("{}",party.key);
            msg!("{}",pda_on);

            program::invoke_signed(
                &instr,
                &[
                    author.clone(), 
                    party.clone()],
                &[
                    &[name.as_bytes(),&[bump],],
                ],
            
            )?;
            msg!("Signed Done");
        } else {
            let required_lamports = rent
                .minimum_balance(Party::LEN)
                .max(1)
                .saturating_sub(current_lamports);

            if required_lamports > 0 {
                let instr = system_instruction::transfer(author.key, party.key, required_lamports);
                
                // ass payer as signer
                program::invoke_signed(
                    &instr,
                    &[author.clone(), party.clone()],
                    &[&[name.as_bytes()], &[&[bump]]],
                )?;
            }
            let instr = system_instruction::allocate(party.key, Party::LEN as u64);

            program::invoke_signed(&instr, &[party.clone()], &[&[name.as_bytes()], &[&[bump]]])?;

            let instr = system_instruction::assign(party.key, program_id);

            program::invoke_signed(&instr, &[party.clone()], &[&[name.as_bytes()], &[&[bump]]])?;
        }

        msg!("Here");


        if pda_on != *party.key {

        }
        if !party.is_writable{

        }
        if rent.is_exempt(party.lamports(), party.try_data_len()?)
        {

        }
        if !author.is_writable
        {

        }



        let mut party_data = party.data.borrow_mut();

        let mut party_state = Party::unpack_unchecked(&party_data)?;

        if party_state.is_initialized() {

            //return Err(SampleError::AlreadyInitializedState.into());
        } else {
            party_state.is_initialized = true;
        }

        //let system_program = next_account_info(accounts_iter)?;


        msg!("Try accounts finished");

        Ok(())
    }

    fn process_create_voter(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ix_data: &[u8],
    ) -> ProgramResult {
        Ok(())
    }

    fn process_vote_positive(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ix_data: &[u8],
    ) -> ProgramResult {
        Ok(())
    }

    fn process_vote_negative(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ix_data: &[u8],
    ) -> ProgramResult {
        Ok(())
    }
}
