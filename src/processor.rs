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
                Self::process_vote_positive(program_id, accounts, ix_data)
            }
            [4] => {
                msg!("Instruction: VoteNegative");
                Self::process_vote_negative(program_id, accounts, ix_data)
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

        if *system_program.key != solana_program::system_program::id() {
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

        let mut owner_state = VotingOwner::unpack_unchecked(&voting_owner.data.borrow_mut())?;

        let mut state_state = VotingState::unpack_unchecked(&voting_state.data.borrow_mut())?;

        if owner_state.is_initialized() || state_state.is_initialized() {
            return Err(JanecekError::AccountAlreadyInitialized.into());
        }

        owner_state.is_initialized = true;
        owner_state.initializer = *initiator.key;
        owner_state.voting_state = *voting_state.key;
        owner_state.bump = bump_owner;

        state_state.is_initialized = true;
        state_state.voting_owner = *initiator.key;

        let clock: Clock = Clock::get().unwrap();
        state_state.voting_started = clock.unix_timestamp;

        match clock.unix_timestamp.checked_add(7 * 24 * 60 * 60) {
            Some(sucess) => state_state.voting_ends = sucess,
            None => return Err(JanecekError::AdditionOverflow.into()),
        }

        state_state.bump = bump_state;

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
                &[&[&name.as_bytes(), pda_state.key.as_ref(), &[bump_party]]],
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
                    &[&[&name.as_bytes(), pda_state.key.as_ref(), &[bump_party]]],
                )?;
            }

            program::invoke_signed(
                &system_instruction::allocate(pda_party.key, Party::LEN as u64),
                &[system_program.clone(), pda_party.clone()],
                &[&[&name.as_bytes(), pda_state.key.as_ref(), &[bump_party]]],
            )?;

            program::invoke_signed(
                &system_instruction::assign(pda_party.key, program_id),
                &[system_program.clone(), pda_party.clone()],
                &[&[&name.as_bytes(), pda_state.key.as_ref(), &[bump_party]]],
            )?;
        }

        // check system program id
        if *system_program.key != solana_program::system_program::id() {
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

        let mut party_state = Party::unpack_unchecked(&pda_party.data.borrow_mut())?;

        if party_state.is_initialized() {
            return Err(JanecekError::AccountAlreadyInitialized.into());
        }

        if !owner_state.is_initialized() || !state_state.is_initialized() {
            return Err(JanecekError::AccountNotInitialized.into());
        }

        if state_state.voting_owner != owner_state.initializer {
            return Err(JanecekError::VotingOwnerMismatch.into());
        }

        // maybe try to perform checked sub and comare to 0
        let clock: Clock = Clock::get().unwrap();
        if clock.unix_timestamp > state_state.voting_ends {
            return Err(JanecekError::VotingEnded.into());
        }

        party_state.is_initialized = true;
        party_state.author = *author.key;
        party_state.voting_state = *pda_state.key;
        party_state.created = clock.unix_timestamp;
        party_state.name = name;
        party_state.votes = 0;
        party_state.bump = bump_party;

        Party::pack(party_state, &mut &mut pda_party.data.borrow_mut()[..])?;

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

        let instruction::CreateVoter {
            bump_owner,
            bump_state,
            bump_voter,
        } = ix;

        let accounts_iter = &mut accounts.iter();

        let author = next_account_info(accounts_iter)?;

        let initiator = next_account_info(accounts_iter)?;

        let pda_owner = next_account_info(accounts_iter)?;

        let pda_state = next_account_info(accounts_iter)?;

        let pda_voter = next_account_info(accounts_iter)?;

        let system_program = next_account_info(accounts_iter)?;

        let (voting_owner_pda, bump_owner_) =
            Pubkey::find_program_address(&[b"voting_owner", initiator.key.as_ref()], program_id);

        let (voting_state_pda, bump_state_) =
            Pubkey::find_program_address(&[b"voting_state", voting_owner_pda.as_ref()], program_id);

        let (voting_voter_pda, bump_voter_) = Pubkey::find_program_address(
            &[b"new_voter", author.key.as_ref(), pda_state.key.as_ref()],
            program_id,
        );

        let rent = Rent::get()?;
        let current_lamports = pda_voter.lamports();
        let lamports_needed = rent.minimum_balance(Voter::LEN);

        if current_lamports == 0 {
            program::invoke_signed(
                &system_instruction::create_account(
                    author.key,
                    pda_voter.key,
                    lamports_needed,
                    Voter::LEN as u64,
                    program_id,
                ),
                &[system_program.clone(), author.clone(), pda_voter.clone()],
                &[&[
                    b"new_voter".as_ref(),
                    author.key.as_ref(),
                    pda_state.key.as_ref(),
                    &[bump_voter],
                ]],
            )?;
        } else {
            let required_lamports = rent
                .minimum_balance(Voter::LEN)
                .max(1)
                .saturating_sub(current_lamports);

            if required_lamports > 0 {
                program::invoke_signed(
                    &system_instruction::transfer(author.key, pda_voter.key, required_lamports),
                    &[system_program.clone(), author.clone(), pda_voter.clone()],
                    &[&[
                        b"new_voter".as_ref(),
                        author.key.as_ref(),
                        pda_state.key.as_ref(),
                        &[bump_voter],
                    ]],
                )?;
            }

            program::invoke_signed(
                &system_instruction::allocate(pda_voter.key, Voter::LEN as u64),
                &[system_program.clone(), pda_voter.clone()],
                &[&[
                    b"new_voter".as_ref(),
                    author.key.as_ref(),
                    pda_state.key.as_ref(),
                    &[bump_voter],
                ]],
            )?;

            program::invoke_signed(
                &system_instruction::assign(pda_voter.key, program_id),
                &[system_program.clone(), pda_voter.clone()],
                &[&[
                    b"new_voter".as_ref(),
                    author.key.as_ref(),
                    pda_state.key.as_ref(),
                    &[bump_voter],
                ]],
            )?;
        }

        // check system program id
        if *system_program.key != solana_program::system_program::id() {
            return Err(JanecekError::AccountNotSystemOwned.into());
        }

        // everyone can add yourself as voter, so owner dont need to sign
        if !author.is_signer {
            return Err(JanecekError::AccountNotSigner.into());
        }

        // check mutables
        if !pda_voter.is_writable {
            return Err(JanecekError::ConstraintMut.into());
        }
        if !author.is_writable {
            return Err(JanecekError::ConstraintMut.into());
        }

        // pda correctness
        if voting_voter_pda != *pda_voter.key || bump_voter_ != bump_voter {
            return Err(JanecekError::PdaMismatch.into());
        }
        if voting_owner_pda != *pda_owner.key || bump_owner_ != bump_owner {
            return Err(JanecekError::PdaMismatch.into());
        }
        if voting_state_pda != *pda_state.key || bump_state_ != bump_state {
            return Err(JanecekError::PdaMismatch.into());
        }

        // check rent exempt
        if !rent.is_exempt(pda_voter.lamports(), pda_voter.try_data_len()?) {
            return Err(JanecekError::ConstraintRentExempt.into());
        }

        // deserialize data and check if both are initialized and if owner match
        let owner_state = VotingOwner::unpack_unchecked(&pda_owner.data.borrow_mut())?;

        let state_state = VotingState::unpack_unchecked(&pda_state.data.borrow_mut())?;

        let mut voter_state = Voter::unpack_unchecked(&pda_voter.data.borrow_mut())?;

        // check if voter initialized already
        if voter_state.is_initialized() {
            return Err(JanecekError::AccountAlreadyInitialized.into());
        }

        // check if state and owner are initialized
        if !owner_state.is_initialized() || !state_state.is_initialized() {
            return Err(JanecekError::AccountNotInitialized.into());
        }

        // check if owner and state correspond to each other
        if state_state.voting_owner != owner_state.initializer {
            return Err(JanecekError::VotingOwnerMismatch.into());
        }

        // maybe try to perform checked sub and comare to 0
        let clock: Clock = Clock::get().unwrap();
        if clock.unix_timestamp > state_state.voting_ends {
            return Err(JanecekError::VotingEnded.into());
        }

        voter_state.is_initialized = true;
        voter_state.author = *author.key;
        voter_state.voting_state = *pda_state.key;
        voter_state.num_votes = 3;
        voter_state.bump = bump_voter;

        Voter::pack(voter_state, &mut &mut pda_voter.data.borrow_mut()[..])?;

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

        let instruction::Vote {
            bump_owner,
            bump_state,
            bump_voter,
            bump_party,
            name,
        } = ix;

        let accounts_iter = &mut accounts.iter();

        let author = next_account_info(accounts_iter)?;

        let initiator = next_account_info(accounts_iter)?;

        let pda_owner = next_account_info(accounts_iter)?;

        let pda_state = next_account_info(accounts_iter)?;

        let pda_voter = next_account_info(accounts_iter)?;

        let pda_party = next_account_info(accounts_iter)?;

        let system_program = next_account_info(accounts_iter)?;

        let (voting_owner_pda, bump_owner_) =
            Pubkey::find_program_address(&[b"voting_owner", initiator.key.as_ref()], program_id);

        let (voting_state_pda, bump_state_) =
            Pubkey::find_program_address(&[b"voting_state", voting_owner_pda.as_ref()], program_id);

        let (voting_voter_pda, bump_voter_) = Pubkey::find_program_address(
            &[b"new_voter", author.key.as_ref(), pda_state.key.as_ref()],
            program_id,
        );

        let (voting_party_pda, bump_party_) =
            Pubkey::find_program_address(&[name.as_bytes(), voting_state_pda.as_ref()], program_id);

        // check system program id
        if *system_program.key != solana_program::system_program::id() {
            return Err(JanecekError::AccountNotSystemOwned.into());
        }

        // check if voter signed
        if !author.is_signer {
            return Err(JanecekError::AccountNotSigner.into());
        }

        // check mutables
        if !pda_voter.is_writable {
            return Err(JanecekError::ConstraintMut.into());
        }
        if !pda_party.is_writable {
            return Err(JanecekError::ConstraintMut.into());
        }

        // check PDA correctness
        if voting_owner_pda != *pda_owner.key || bump_owner_ != bump_owner {
            return Err(JanecekError::PdaMismatch.into());
        }
        if voting_state_pda != *pda_state.key || bump_state_ != bump_state {
            return Err(JanecekError::PdaMismatch.into());
        }
        if voting_voter_pda != *pda_voter.key || bump_voter_ != bump_voter {
            return Err(JanecekError::PdaMismatch.into());
        }
        if voting_party_pda != *pda_party.key || bump_party_ != bump_party {
            return Err(JanecekError::PdaMismatch.into());
        }

        let mut voter_state = Voter::unpack_unchecked(&pda_voter.data.borrow_mut())?;

        let mut party_state = Party::unpack_unchecked(&pda_party.data.borrow_mut())?;

        let owner_state = VotingOwner::unpack_unchecked(&pda_owner.data.borrow_mut())?;

        let state_state = VotingState::unpack_unchecked(&pda_state.data.borrow_mut())?;

        // check if voter initialized
        if !voter_state.is_initialized() {
            return Err(JanecekError::AccountNotInitialized.into());
        }

        // check if party initialized
        if !party_state.is_initialized() {
            return Err(JanecekError::AccountNotInitialized.into());
        }

        // check if state and owner are initialized
        if !owner_state.is_initialized() || !state_state.is_initialized() {
            return Err(JanecekError::AccountNotInitialized.into());
        }

        // check if owner and state correspond to each other
        if state_state.voting_owner != owner_state.initializer {
            return Err(JanecekError::VotingOwnerMismatch.into());
        }

        // check if voter exists in this voting state
        if *pda_state.key != voter_state.voting_state {
            return Err(JanecekError::VotingStateMismatch.into());
        }

        // check if party exists in this voting state
        if *pda_state.key != party_state.voting_state {
            return Err(JanecekError::VotingStateMismatch.into());
        }

        // maybe try to perform checked sub and compare to 0
        let clock: Clock = Clock::get().unwrap();
        if clock.unix_timestamp > state_state.voting_ends {
            return Err(JanecekError::VotingEnded.into());
        }

        if voter_state.num_votes == 3 {
            voter_state.num_votes -= 1;
            voter_state.pos1 = *pda_party.key;

            match party_state.votes.checked_add(1) {
                Some(sucess) => party_state.votes = sucess,
                None => return Err(JanecekError::AdditionOverflow.into()),
            }

            Voter::pack(voter_state, &mut &mut pda_voter.data.borrow_mut()[..])?;
            Party::pack(party_state, &mut &mut pda_party.data.borrow_mut()[..])?;

            Ok(())
        } else if voter_state.num_votes == 2 {
            if voter_state.pos1 == *pda_party.key {
                return Err(JanecekError::NoBothPosSameParty.into());
            } else {
                voter_state.num_votes -= 1;
                voter_state.pos2 = *pda_party.key;

                match party_state.votes.checked_add(1) {
                    Some(sucess) => party_state.votes = sucess,
                    None => return Err(JanecekError::AdditionOverflow.into()),
                }

                Voter::pack(voter_state, &mut &mut pda_voter.data.borrow_mut()[..])?;
                Party::pack(party_state, &mut &mut pda_party.data.borrow_mut()[..])?;

                Ok(())
            }
        } else {
            return Err(JanecekError::VotesOutOfRange.into());
        }
    }
    #[allow(dead_code)]

    fn process_vote_negative(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ix_data: &[u8],
    ) -> ProgramResult {
        let ix = instruction::Vote::deserialize(&mut &ix_data[..])
            .map_err(|_| JanecekError::InstructionDidNotDeserialize)?;

        let instruction::Vote {
            bump_owner,
            bump_state,
            bump_voter,
            bump_party,
            name,
        } = ix;

        let accounts_iter = &mut accounts.iter();

        let author = next_account_info(accounts_iter)?;

        let initiator = next_account_info(accounts_iter)?;

        let pda_owner = next_account_info(accounts_iter)?;

        let pda_state = next_account_info(accounts_iter)?;

        let pda_voter = next_account_info(accounts_iter)?;

        let pda_party = next_account_info(accounts_iter)?;

        let system_program = next_account_info(accounts_iter)?;

        let (voting_owner_pda, bump_owner_) =
            Pubkey::find_program_address(&[b"voting_owner", initiator.key.as_ref()], program_id);

        let (voting_state_pda, bump_state_) =
            Pubkey::find_program_address(&[b"voting_state", voting_owner_pda.as_ref()], program_id);

        let (voting_voter_pda, bump_voter_) = Pubkey::find_program_address(
            &[b"new_voter", author.key.as_ref(), pda_state.key.as_ref()],
            program_id,
        );

        let (voting_party_pda, bump_party_) =
            Pubkey::find_program_address(&[name.as_bytes(), voting_state_pda.as_ref()], program_id);

        // check system program id
        if *system_program.key != solana_program::system_program::id() {
            return Err(JanecekError::AccountNotSystemOwned.into());
        }

        // check if voter signed
        if !author.is_signer {
            return Err(JanecekError::AccountNotSigner.into());
        }

        // check mutables
        if !pda_voter.is_writable {
            return Err(JanecekError::ConstraintMut.into());
        }
        if !pda_party.is_writable {
            return Err(JanecekError::ConstraintMut.into());
        }

        // check PDA correctness
        if voting_owner_pda != *pda_owner.key || bump_owner_ != bump_owner {
            return Err(JanecekError::PdaMismatch.into());
        }
        if voting_state_pda != *pda_state.key || bump_state_ != bump_state {
            return Err(JanecekError::PdaMismatch.into());
        }
        if voting_voter_pda != *pda_voter.key || bump_voter_ != bump_voter {
            return Err(JanecekError::PdaMismatch.into());
        }
        if voting_party_pda != *pda_party.key || bump_party_ != bump_party {
            return Err(JanecekError::PdaMismatch.into());
        }

        let mut voter_state = Voter::unpack_unchecked(&pda_voter.data.borrow_mut())?;

        let mut party_state = Party::unpack_unchecked(&pda_party.data.borrow_mut())?;

        let owner_state = VotingOwner::unpack_unchecked(&pda_owner.data.borrow_mut())?;

        let state_state = VotingState::unpack_unchecked(&pda_state.data.borrow_mut())?;

        // check if voter initialized
        if !voter_state.is_initialized() {
            return Err(JanecekError::AccountNotInitialized.into());
        }

        // check if party initialized
        if !party_state.is_initialized() {
            return Err(JanecekError::AccountNotInitialized.into());
        }

        // check if state and owner are initialized
        if !owner_state.is_initialized() || !state_state.is_initialized() {
            return Err(JanecekError::AccountNotInitialized.into());
        }

        // check if owner and state correspond to each other
        if state_state.voting_owner != owner_state.initializer {
            return Err(JanecekError::VotingOwnerMismatch.into());
        }

        // check if voter exists in this voting state
        if *pda_state.key != voter_state.voting_state {
            return Err(JanecekError::VotingStateMismatch.into());
        }

        // check if party exists in this voting state
        if *pda_state.key != party_state.voting_state {
            return Err(JanecekError::VotingStateMismatch.into());
        }

        // maybe try to perform checked sub and compare to 0
        let clock: Clock = Clock::get().unwrap();
        if clock.unix_timestamp > state_state.voting_ends {
            return Err(JanecekError::VotingEnded.into());
        }

        if voter_state.num_votes == 1 {
            voter_state.num_votes -= 1;
            voter_state.neg1 = *pda_party.key;

            match party_state.votes.checked_sub(1) {
                Some(sucess) => party_state.votes = sucess,
                None => return Err(JanecekError::SubtractionOverflow.into()),
            }

            Voter::pack(voter_state, &mut &mut pda_voter.data.borrow_mut()[..])?;
            Party::pack(party_state, &mut &mut pda_party.data.borrow_mut()[..])?;

            Ok(())
        }else {
            return Err(JanecekError::VotesOutOfRange.into());
        }
    }
}
