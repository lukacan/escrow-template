use borsh::BorshDeserialize;
use solana_program::account_info::next_account_info;

use solana_program::clock::Clock;
use solana_program::program;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
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

        let instruction::CreateParty { name, bump } = ix;

        let accounts_iter = &mut accounts.iter();

        let author = next_account_info(accounts_iter)?;

        let party = next_account_info(accounts_iter)?;

        let system_program = next_account_info(accounts_iter)?;

        let (pda, _bump) = Pubkey::find_program_address(&[name.as_bytes()], program_id);

        let rent = Rent::get()?;
        let current_lamports = party.lamports();
        let lamports_needed = rent.minimum_balance(Party::LEN);

        if current_lamports == 0 {
            program::invoke_signed(
                &system_instruction::create_account(
                    author.key,
                    party.key,
                    lamports_needed,
                    Party::LEN as u64,
                    program_id,
                ),
                &[system_program.clone(), author.clone(), party.clone()],
                &[&[name.as_bytes(), &[bump]]],
            )?;
        } else {
            let required_lamports = rent
                .minimum_balance(Party::LEN)
                .max(1)
                .saturating_sub(current_lamports);

            if required_lamports > 0 {
                program::invoke_signed(
                    &system_instruction::transfer(author.key, party.key, required_lamports),
                    &[system_program.clone(), author.clone(), party.clone()],
                    &[&[name.as_bytes()], &[&[bump]]],
                )?;
            }

            program::invoke_signed(
                &system_instruction::allocate(party.key, Party::LEN as u64),
                &[system_program.clone(), party.clone()],
                &[&[name.as_bytes()], &[&[bump]]],
            )?;

            program::invoke_signed(
                &system_instruction::assign(party.key, program_id),
                &[system_program.clone(), party.clone()],
                &[&[name.as_bytes()], &[&[bump]]],
            )?;
        }

        if !author.is_signer {
            return Err(JanecekError::AccountNotSigner.into());
        }

        if !author.is_writable {}

        if !party.is_writable {}

        if pda != *party.key || _bump != bump {
            return Err(JanecekError::PdaMismatch.into());
        }
        if !party.is_writable {
            return Err(JanecekError::ConstraintMut.into());
        }
        if !author.is_writable {
            return Err(JanecekError::ConstraintMut.into());
        }
        if !rent.is_exempt(party.lamports(), party.try_data_len()?) {
            return Err(JanecekError::ConstraintRentExempt.into());
        }
        let mut party_state = Party::unpack_unchecked(&party.data.borrow_mut())?;

        if party_state.is_initialized() {
            return Err(JanecekError::AccountAlreadyInitialized.into());
        } else {
            let clock: Clock = Clock::get().unwrap();
            party_state.is_initialized = true;
            party_state.author = *author.key;
            party_state.created = clock.unix_timestamp;
            party_state.voting_ends = clock.unix_timestamp + (7 * 24 * 60 * 60);
            party_state.name = name;
            party_state.votes = 0;
            party_state.bump = bump;

            Party::pack(party_state, &mut &mut party.data.borrow_mut()[..])?;
        }

        let party_state = Party::unpack(&party.data.borrow_mut())?;
        msg!("{}", author.key);
        msg!("{}", party_state.author);
        msg!("{}", party_state.created);
        msg!("{}", party_state.voting_ends);
        msg!("{}", party_state.name);
        msg!("{}", party_state.name.chars().count());
        msg!("{}", party_state.votes);
        msg!("{}", party_state.bump);

        Ok(())
    }

    #[allow(dead_code)]
    fn process_create_voter(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _ix_data: &[u8],
    ) -> ProgramResult {
        Ok(())
    }
    #[allow(dead_code)]
    fn process_vote_positive(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _ix_data: &[u8],
    ) -> ProgramResult {
        Ok(())
    }
    #[allow(dead_code)]
    fn process_vote_negative(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _ix_data: &[u8],
    ) -> ProgramResult {
        Ok(())
    }
}
