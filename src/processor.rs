use borsh::BorshDeserialize;
use solana_program::account_info::next_account_info;



use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
    program_pack::{Pack, IsInitialized}
};



use crate::{error::JanecekError, 
    instruction::instruction};
use crate::identifier::ID;



pub struct Processor;
impl Processor {
    pub fn entry(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        Self::try_entry(program_id, accounts, instruction_data)
        .map_err(|e| {
            //e.log();
            e.into()
        })
    }
    fn try_entry(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
    ) -> ProgramResult {
        
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

    fn dispatch(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
    ) -> ProgramResult {
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
    )->ProgramResult {
        let ix = instruction::CreateParty::deserialize(
            &mut &ix_data[..]
        )
        .map_err(|_| {
            JanecekError::InstructionDidNotDeserialize
        })?;

        msg!("Instruction data Deserialized");
        let instruction::CreateParty { name } = ix;

        let mut bumps = std::collections::BTreeMap::<String,u8>::new();

        let mut remaining_accounts: &[AccountInfo] = accounts;


        //here we will call create_party which will work with context

        Ok(())
    }

    fn process_create_voter(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ix_data: &[u8],
    )->ProgramResult {
        Ok(())
    }

    fn process_vote_positive(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ix_data: &[u8],
    )->ProgramResult {
        Ok(())
    }

    fn process_vote_negative(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ix_data: &[u8],
    )->ProgramResult {
        Ok(())
    }
}
