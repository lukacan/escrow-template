use borsh::BorshDeserialize;
use solana_program::account_info::next_account_info;

use crate::state::state::{Party, Voter, VotingOwner, VotingState};
use crate::{error::JanecekError, instruction::instruction};
use solana_program::clock::Clock;
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
use solana_program::{declare_id, program};

declare_id!("Fnambs3f1XXoMmAVc94bf8t6JDAxmVkXz85XU4v2edph");

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
        if *program_id != id() {
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
                msg!("Instruction: Initialize");
                Self::process_initialize(program_id, accounts, ix_data)
            }
            [1] => {
                msg!("Instruction: CreateParty");
                Self::process_create_party(program_id, accounts, ix_data)
            }
            [2] => {
                msg!("Instruction: CreateVoter");
                Self::process_create_voter(program_id, accounts, ix_data)
            }
            [3] => {
                msg!("Instruction: VotePositive");
                Self::process_create_party(program_id, accounts, ix_data)
            }
            [4] => {
                msg!("Instruction: VoteNegative");
                Self::process_create_party(program_id, accounts, ix_data)
            }
            _ => Err(JanecekError::InstructionFallbackNotFound.into()),
        }
    }

    fn process_initialize(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ix_data: &[u8],
    ) -> ProgramResult {
        let ix = instruction::Initialize::deserialize(&mut &ix_data[..])
            .map_err(|_| JanecekError::InstructionDidNotDeserialize)?;

        let instruction::Initialize {
            bump_owner,
            bump_state,
        } = ix;

        let accounts_iter = &mut accounts.iter();

        let initiator = next_account_info(accounts_iter)?;

        let voting_owner = next_account_info(accounts_iter)?;

        let voting_state = next_account_info(accounts_iter)?;

        let system_program = next_account_info(accounts_iter)?;

        let (voting_owner_pda, bump_owner_) =
            Pubkey::find_program_address(&[b"voting_owner", initiator.key.as_ref()], program_id);

        let (voting_state_pda, bump_state_) =
            Pubkey::find_program_address(&[b"voting_state", voting_owner.key.as_ref()], program_id);

        let rent = Rent::get()?;
        let current_lamports = voting_owner.lamports();
        let lamports_needed = rent.minimum_balance(VotingOwner::LEN);

        if current_lamports == 0 {
            program::invoke_signed(
                &system_instruction::create_account(
                    initiator.key,
                    voting_owner.key,
                    lamports_needed,
                    VotingOwner::LEN as u64,
                    program_id,
                ),
                &[
                    system_program.clone(),
                    initiator.clone(),
                    voting_owner.clone(),
                ],
                &[&[
                    b"voting_owner".as_ref(),
                    initiator.key.as_ref(),
                    &[bump_owner],
                ]],
            )?;
        } else {
            let required_lamports = rent
                .minimum_balance(VotingOwner::LEN)
                .max(1)
                .saturating_sub(current_lamports);

            if required_lamports > 0 {
                program::invoke_signed(
                    &system_instruction::transfer(
                        initiator.key,
                        voting_owner.key,
                        required_lamports,
                    ),
                    &[
                        system_program.clone(),
                        initiator.clone(),
                        voting_owner.clone(),
                    ],
                    &[&[
                        b"voting_owner".as_ref(),
                        initiator.key.as_ref(),
                        &[bump_owner],
                    ]],
                )?;
            }

            program::invoke_signed(
                &system_instruction::allocate(voting_owner.key, Party::LEN as u64),
                &[system_program.clone(), voting_owner.clone()],
                &[&[
                    b"voting_owner".as_ref(),
                    initiator.key.as_ref(),
                    &[bump_owner],
                ]],
            )?;

            program::invoke_signed(
                &system_instruction::assign(voting_owner.key, program_id),
                &[system_program.clone(), voting_owner.clone()],
                &[&[
                    b"voting_owner".as_ref(),
                    initiator.key.as_ref(),
                    &[bump_owner],
                ]],
            )?;
        }

        let current_lamports = voting_state.lamports();
        let lamports_needed = rent.minimum_balance(VotingState::LEN);

        if current_lamports == 0 {
            program::invoke_signed(
                &system_instruction::create_account(
                    initiator.key,
                    voting_state.key,
                    lamports_needed,
                    VotingState::LEN as u64,
                    program_id,
                ),
                &[
                    system_program.clone(),
                    initiator.clone(),
                    voting_state.clone(),
                ],
                &[&[
                    b"voting_state".as_ref(),
                    voting_owner.key.as_ref(),
                    &[bump_state],
                ]],
            )?;
        } else {
            let required_lamports = rent
                .minimum_balance(VotingState::LEN)
                .max(1)
                .saturating_sub(current_lamports);

            if required_lamports > 0 {
                program::invoke_signed(
                    &system_instruction::transfer(
                        initiator.key,
                        voting_state.key,
                        required_lamports,
                    ),
                    &[
                        system_program.clone(),
                        initiator.clone(),
                        voting_state.clone(),
                    ],
                    &[&[
                        b"voting_state".as_ref(),
                        voting_owner.key.as_ref(),
                        &[bump_state],
                    ]],
                )?;
            }

            program::invoke_signed(
                &system_instruction::allocate(voting_state.key, VotingState::LEN as u64),
                &[system_program.clone(), voting_state.clone()],
                &[&[
                    b"voting_state".as_ref(),
                    voting_owner.key.as_ref(),
                    &[bump_state],
                ]],
            )?;

            program::invoke_signed(
                &system_instruction::assign(voting_state.key, program_id),
                &[system_program.clone(), voting_state.clone()],
                &[&[
                    b"voting_state".as_ref(),
                    voting_owner.key.as_ref(),
                    &[bump_state],
                ]],
            )?;
        }

        
        
        if *system_program.key != solana_program::system_program::id(){
            return Err(JanecekError::AccountNotSystemOwned.into());
        }
        
        
        // check provided PDAs and bumps match
        if voting_owner_pda != *voting_owner.key || bump_owner_ != bump_owner {
            return Err(JanecekError::PdaMismatch.into());
        }
        if voting_state_pda != *voting_state.key || bump_state_ != bump_state {
            return Err(JanecekError::PdaMismatch.into());
        }

        // check if initiator is signer
        if !initiator.is_signer {
            return Err(JanecekError::AccountNotSigner.into());
        }

        // check mutable state
        if !initiator.is_writable {
            return Err(JanecekError::ConstraintMut.into());
        }
        if !voting_state.is_writable {
            return Err(JanecekError::ConstraintMut.into());
        }
        if !voting_owner.is_writable {
            return Err(JanecekError::ConstraintMut.into());
        }

        // check rent exempt
        if !rent.is_exempt(voting_state.lamports(), voting_state.try_data_len()?) {
            return Err(JanecekError::ConstraintRentExempt.into());
        }
        if !rent.is_exempt(voting_owner.lamports(), voting_owner.try_data_len()?) {
            return Err(JanecekError::ConstraintRentExempt.into());
        }

        // update owner and state

        let owner_state = VotingOwner::unpack_unchecked(&voting_owner.data.borrow_mut())?;

        let state_state = VotingState::unpack_unchecked(&voting_state.data.borrow_mut())?;


        VotingOwner::pack(owner_state, &mut &mut voting_owner.data.borrow_mut()[..])?;
        VotingState::pack(state_state, &mut &mut voting_state.data.borrow_mut()[..])?;



        Ok(())
    }

    fn process_create_party(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ix_data: &[u8],
    ) -> ProgramResult {
        let ix = instruction::CreateParty::deserialize(&mut &ix_data[..])
            .map_err(|_| JanecekError::InstructionDidNotDeserialize)?;

        let instruction::CreateParty {
            bump_owner,
            bump_state,
            bump_party,
            name,
        } = ix;

        let accounts_iter = &mut accounts.iter();

        let author = next_account_info(accounts_iter)?;

        let initiator = next_account_info(accounts_iter)?;

        let pda_owner = next_account_info(accounts_iter)?;

        let pda_state = next_account_info(accounts_iter)?;

        let pda_party = next_account_info(accounts_iter)?;

        let system_program = next_account_info(accounts_iter)?;

        let (voting_owner_pda, bump_owner_) =
            Pubkey::find_program_address(&[b"voting_owner", initiator.key.as_ref()], program_id);

        let (voting_state_pda, bump_state_) =
            Pubkey::find_program_address(&[b"voting_state", voting_owner_pda.as_ref()], program_id);

        let (voting_party_pda, bump_party_) =
            Pubkey::find_program_address(&[name.as_bytes(), voting_state_pda.as_ref()], program_id);



        let rent = Rent::get()?;
        let current_lamports = pda_party.lamports();
        let lamports_needed = rent.minimum_balance(Party::LEN);

        if current_lamports == 0 {
            program::invoke_signed(
                &system_instruction::create_account(
                    author.key,
                    pda_party.key,
                    lamports_needed,
                    Party::LEN as u64,
                    program_id,
                ),
                &[system_program.clone(), author.clone(), pda_party.clone()],
                &[&[&name.as_bytes(),pda_state.key.as_ref(), &[bump_party]]],
            )?;
        } else {
            let required_lamports = rent
                .minimum_balance(Party::LEN)
                .max(1)
                .saturating_sub(current_lamports);

            if required_lamports > 0 {
                program::invoke_signed(
                    &system_instruction::transfer(author.key, pda_party.key, required_lamports),
                    &[system_program.clone(), author.clone(), pda_party.clone()],
                    &[&[&name.as_bytes(),pda_state.key.as_ref(), &[bump_party]]],
                )?;
            }

            program::invoke_signed(
                &system_instruction::allocate(pda_party.key, Party::LEN as u64),
                &[system_program.clone(), pda_party.clone()],
                &[&[&name.as_bytes(),pda_state.key.as_ref(), &[bump_party]]],
            )?;

            program::invoke_signed(
                &system_instruction::assign(pda_party.key, program_id),
                &[system_program.clone(), pda_party.clone()],
                &[&[&name.as_bytes(),pda_state.key.as_ref(), &[bump_party]]],
            )?;
        }
        
        
        
        if *system_program.key != solana_program::system_program::id(){
            return Err(JanecekError::AccountNotSystemOwned.into());
        }
        
        
        // check signers
        if !initiator.is_signer {
            return Err(JanecekError::AccountNotSigner.into());
        }
        if !author.is_signer {
            return Err(JanecekError::AccountNotSigner.into());
        }

        // check mutables
        if !pda_party.is_writable {
            return Err(JanecekError::ConstraintMut.into());
        }
        if !author.is_writable {
            return Err(JanecekError::ConstraintMut.into());
        }


        // pda correctness
        if voting_party_pda != *pda_party.key || bump_party_ != bump_party {
            return Err(JanecekError::PdaMismatch.into());
        }
        if voting_owner_pda != *pda_owner.key || bump_owner_ != bump_owner {
            return Err(JanecekError::PdaMismatch.into());
        }
        if voting_state_pda != *pda_state.key || bump_state_ != bump_state {
            return Err(JanecekError::PdaMismatch.into());
        }

        // check rent exempt
        if !rent.is_exempt(pda_party.lamports(), pda_party.try_data_len()?) {
            return Err(JanecekError::ConstraintRentExempt.into());
        }


        
        // deserialize data and check if both are initialized and if owner match
        let owner_state = VotingOwner::unpack_unchecked(&pda_owner.data.borrow_mut())?;

        let state_state = VotingState::unpack_unchecked(&pda_state.data.borrow_mut())?;


        VotingOwner::pack(owner_state, &mut &mut pda_owner.data.borrow_mut()[..])?;
        VotingState::pack(state_state, &mut &mut pda_state.data.borrow_mut()[..])?;
        
        
        let mut party_state = Party::unpack_unchecked(&pda_party.data.borrow_mut())?;

        if party_state.is_initialized() {
            return Err(JanecekError::AccountAlreadyInitialized.into());
        } else {
            let clock: Clock = Clock::get().unwrap();
            party_state.is_initialized = true;
            party_state.author = *author.key;
            party_state.voting_state = *pda_state.key;
            party_state.created = clock.unix_timestamp;
            party_state.voting_ends = clock.unix_timestamp + (7 * 24 * 60 * 60);
            party_state.name = name;
            party_state.votes = 0;
            party_state.bump = bump_party;

            Party::pack(party_state, &mut &mut pda_party.data.borrow_mut()[..])?;
        }

        // let party_state = Party::unpack(&party.data.borrow_mut())?;
        // msg!("Author: {}", author.key);
        // msg!("Party: {}", party_state.author);
        // msg!("Party: {}", party_state.created);
        // msg!("Party: {}", party_state.voting_ends);
        // msg!("Party: {}", party_state.name);
        // msg!("Party: {}", party_state.name.chars().count());
        // msg!("Party: {}", party_state.votes);
        // msg!("Party: {}", party_state.bump);



        

        Ok(())
    }

    #[allow(dead_code)]
    fn process_create_voter(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ix_data: &[u8],
    ) -> ProgramResult {
        let ix = instruction::CreateVoter::deserialize(&mut &ix_data[..])
            .map_err(|_| JanecekError::InstructionDidNotDeserialize)?;

        let instruction::CreateVoter { bump } = ix;

        let accounts_iter = &mut accounts.iter();

        let author = next_account_info(accounts_iter)?;

        let voter = next_account_info(accounts_iter)?;

        let system_program = next_account_info(accounts_iter)?;

        let (pda, _bump) =
            Pubkey::find_program_address(&[b"new_voter", author.key.as_ref()], program_id);

        let rent = Rent::get()?;
        let current_lamports = voter.lamports();
        let lamports_needed = rent.minimum_balance(Voter::LEN);

        if current_lamports == 0 {
            program::invoke_signed(
                &system_instruction::create_account(
                    author.key,
                    voter.key,
                    lamports_needed,
                    Voter::LEN as u64,
                    program_id,
                ),
                &[system_program.clone(), author.clone(), voter.clone()],
                &[&[b"new_voter".as_ref(), author.key.as_ref(), &[bump]]],
            )?;
        } else {
            let required_lamports = rent
                .minimum_balance(Voter::LEN)
                .max(1)
                .saturating_sub(current_lamports);

            if required_lamports > 0 {
                program::invoke_signed(
                    &system_instruction::transfer(author.key, voter.key, required_lamports),
                    &[system_program.clone(), author.clone(), voter.clone()],
                    &[&[b"new_voter".as_ref(), author.key.as_ref(), &[bump]]],
                )?;
            }

            program::invoke_signed(
                &system_instruction::allocate(voter.key, Voter::LEN as u64),
                &[system_program.clone(), voter.clone()],
                &[&[b"new_voter".as_ref(), author.key.as_ref(), &[bump]]],
            )?;

            program::invoke_signed(
                &system_instruction::assign(voter.key, program_id),
                &[system_program.clone(), voter.clone()],
                &[&[b"new_voter".as_ref(), author.key.as_ref(), &[bump]]],
            )?;
        }

        if *system_program.key != solana_program::system_program::id() {
            return Err(JanecekError::AccountNotSystemOwned.into());
        }
        if !author.is_signer {
            return Err(JanecekError::AccountNotSigner.into());
        }
        if !voter.is_writable {
            return Err(JanecekError::ConstraintMut.into());
        }
        if !author.is_writable {
            return Err(JanecekError::ConstraintMut.into());
        }
        if pda != *voter.key || _bump != bump {
            return Err(JanecekError::PdaMismatch.into());
        }
        if !rent.is_exempt(voter.lamports(), voter.try_data_len()?) {
            return Err(JanecekError::ConstraintRentExempt.into());
        }
        let mut voter_state = Voter::unpack_unchecked(&voter.data.borrow_mut())?;

        if voter_state.is_initialized() {
            return Err(JanecekError::AccountAlreadyInitialized.into());
        } else {
            voter_state.is_initialized = true;
            voter_state.author = *author.key;
            voter_state.num_votes = 3;
            voter_state.bump = bump;

            Voter::pack(voter_state, &mut &mut voter.data.borrow_mut()[..])?;
        }

        // let voter_state = Voter::unpack(&voter.data.borrow_mut())?;
        // msg!("Author {}", author.key);
        // msg!("Voter {}", voter_state.author);
        // msg!("Voter {}", voter_state.num_votes);
        // msg!("Voter {}", voter_state.pos1);
        // msg!("Voter {}", voter_state.pos2);
        // msg!("Voter {}", voter_state.neg1);
        // msg!("Voter {}", voter_state.bump);

        Ok(())
    }
    #[allow(dead_code)]
    fn process_vote_positive(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ix_data: &[u8],
    ) -> ProgramResult {
        let ix = instruction::Vote::deserialize(&mut &ix_data[..])
            .map_err(|_| JanecekError::InstructionDidNotDeserialize)?;

        let instruction::Vote { bump } = ix;

        let accounts_iter = &mut accounts.iter();

        let author = next_account_info(accounts_iter)?;

        let voter = next_account_info(accounts_iter)?;

        let party = next_account_info(accounts_iter)?;

        let _system_program = next_account_info(accounts_iter)?;

        let (pda, _bump) =
            Pubkey::find_program_address(&[b"new_voter", author.key.as_ref()], program_id);

        let mut voter_state = Voter::unpack_unchecked(&voter.data.borrow_mut())?;

        let mut party_state = Party::unpack_unchecked(&party.data.borrow_mut())?;

        if pda != *voter.key || _bump != bump {
            return Err(JanecekError::PdaMismatch.into());
        }
        if !voter_state.is_initialized {
            return Err(JanecekError::AccountNotInitialized.into());
        }
        if !voter.is_writable {
            return Err(JanecekError::ConstraintMut.into());
        }
        if !author.is_signer {
            return Err(JanecekError::AccountNotSigner.into());
        }
        if !voter_state.is_initialized {
            return Err(JanecekError::AccountNotInitialized.into());
        }
        if !party.is_writable {
            return Err(JanecekError::ConstraintMut.into());
        }

        let clock: Clock = Clock::get().unwrap();
        if clock.unix_timestamp > party_state.voting_ends {
            return Err(JanecekError::VotingEnded.into());
        }

        if voter_state.num_votes == 3 {
            voter_state.num_votes -= 1;
            voter_state.pos1 = *party.key;

            match party_state.votes.checked_add(1) {
                Some(sucess) => party_state.votes = sucess,
                None => return Err(JanecekError::PosOverflow.into()),
            }

            Voter::pack(voter_state, &mut &mut voter.data.borrow_mut()[..])?;
            Party::pack(party_state, &mut &mut party.data.borrow_mut()[..])?;

            Ok(())
        } else if voter_state.num_votes == 2 {
            if voter_state.pos1 == *party.key {
                return Err(JanecekError::NoBothPosSameParty.into());
            } else {
                voter_state.num_votes -= 1;
                voter_state.pos2 = *party.key;

                match party_state.votes.checked_add(1) {
                    Some(sucess) => party_state.votes = sucess,
                    None => return Err(JanecekError::PosOverflow.into()),
                }

                Voter::pack(voter_state, &mut &mut voter.data.borrow_mut()[..])?;
                Party::pack(party_state, &mut &mut party.data.borrow_mut()[..])?;

                Ok(())
            }
        } else {
            return Err(JanecekError::VotesOutOfRange.into());
        }
    }
    #[allow(dead_code)]

    fn process_vote_negative(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _ix_data: &[u8],
    ) -> ProgramResult {
        todo!()
    }
}
