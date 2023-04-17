use crate::state::state::{
    Party, Voter, VotingOwner, VotingState, PARTY, VOTER, VOTINGOWNER, VOTINGSTATE, NAME_LENGTH,
};
use crate::{error::JanecekError, instruction::instruction};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
    {declare_id, program},
};

declare_id!("Fnambs3f1XXoMmAVc94bf8t6JDAxmVkXz85XU4v2edph");

pub struct Processor;
impl Processor {
    pub fn entry(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        Self::try_entry(program_id, accounts, instruction_data)
    }
    fn try_entry(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
        // check if program ID is correct
        if *program_id != id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        // check if data contains at least 1 byte, so function can be decoded
        if data.len() < 1 {
            return Err(ProgramError::InvalidInstructionData);
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
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }

    fn process_initialize(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ix_data: &[u8],
    ) -> ProgramResult {
        let ix = instruction::Initialize::deserialize(&mut &ix_data[..])
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        let instruction::Initialize {
            bump_owner,
            bump_state,
        } = ix;

        let accounts_iter = &mut accounts.iter();

        let author = next_account_info(accounts_iter)?;

        let pda_owner = next_account_info(accounts_iter)?;

        let pda_state = next_account_info(accounts_iter)?;

        let system_program = next_account_info(accounts_iter)?;

        let (pda_owner_, bump_owner_) =
            Pubkey::find_program_address(&[b"voting_owner", author.key.as_ref()], program_id);

        let (pda_state_, bump_state_) =
            Pubkey::find_program_address(&[b"voting_state", pda_owner.key.as_ref()], program_id);

        let rent = Rent::get()?;
        let current_lamports = pda_owner.lamports();
        let lamports_needed = rent.minimum_balance(VotingOwner::LEN);

        if current_lamports == 0 {
            program::invoke_signed(
                &system_instruction::create_account(
                    author.key,
                    pda_owner.key,
                    lamports_needed,
                    VotingOwner::LEN as u64,
                    program_id,
                ),
                &[author.clone(), pda_owner.clone()],
                &[&[b"voting_owner".as_ref(), author.key.as_ref(), &[bump_owner]]],
            )?;
        } else {
            let required_lamports = rent
                .minimum_balance(VotingOwner::LEN)
                .max(1)
                .saturating_sub(current_lamports);

            if required_lamports > 0 {
                program::invoke_signed(
                    &system_instruction::transfer(author.key, &pda_owner.key, required_lamports),
                    &[author.clone(), pda_owner.clone()],
                    &[&[b"voting_owner".as_ref(), author.key.as_ref(), &[bump_owner]]],
                )?;
            }

            program::invoke_signed(
                &system_instruction::allocate(pda_owner.key, Party::LEN as u64),
                &[pda_owner.clone()],
                &[&[b"voting_owner".as_ref(), author.key.as_ref(), &[bump_owner]]],
            )?;

            program::invoke_signed(
                &system_instruction::assign(pda_owner.key, program_id),
                &[pda_owner.clone()],
                &[&[b"voting_owner".as_ref(), author.key.as_ref(), &[bump_owner]]],
            )?;
        }

        let current_lamports = pda_state.lamports();
        let lamports_needed = rent.minimum_balance(VotingState::LEN);

        if current_lamports == 0 {
            program::invoke_signed(
                &system_instruction::create_account(
                    author.key,
                    pda_state.key,
                    lamports_needed,
                    VotingState::LEN as u64,
                    program_id,
                ),
                &[author.clone(), pda_state.clone()],
                &[&[
                    b"voting_state".as_ref(),
                    pda_owner.key.as_ref(),
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
                    &system_instruction::transfer(author.key, pda_state.key, required_lamports),
                    &[author.clone(), pda_state.clone()],
                    &[&[
                        b"voting_state".as_ref(),
                        pda_owner.key.as_ref(),
                        &[bump_state],
                    ]],
                )?;
            }

            program::invoke_signed(
                &system_instruction::allocate(pda_state.key, VotingState::LEN as u64),
                &[pda_state.clone()],
                &[&[
                    b"voting_state".as_ref(),
                    pda_owner.key.as_ref(),
                    &[bump_state],
                ]],
            )?;

            program::invoke_signed(
                &system_instruction::assign(pda_state.key, program_id),
                &[pda_state.clone()],
                &[&[
                    b"voting_state".as_ref(),
                    pda_owner.key.as_ref(),
                    &[bump_state],
                ]],
            )?;
        }

        // SIGNER CHECK
        if !author.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // PROGRAM OWNERS CHECK
        if *pda_owner.owner != id() || *pda_state.owner != id() {
            return Err(ProgramError::IllegalOwner);
        }

        // SYSTEM PROGRAM ID
        if *system_program.key != solana_program::system_program::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        // writable as payer
        if !author.is_writable {
            return Err(JanecekError::AccountNotmutable.into());
        }
        if !pda_state.is_writable {
            return Err(JanecekError::AccountNotmutable.into());
        }
        if !pda_owner.is_writable {
            return Err(JanecekError::AccountNotmutable.into());
        }
        if system_program.is_writable {
            return Err(JanecekError::AccountMutable.into());
        }

        // check provided PDAs and bumps match
        if pda_owner_ != *pda_owner.key || bump_owner_ != bump_owner {
            return Err(ProgramError::InvalidSeeds);
        }
        if pda_state_ != *pda_state.key || bump_state_ != bump_state {
            return Err(ProgramError::InvalidSeeds);
        }

        // double check rent exempt
        if !rent.is_exempt(pda_state.lamports(), pda_state.try_data_len()?) {
            return Err(ProgramError::AccountNotRentExempt);
        }
        if !rent.is_exempt(pda_owner.lamports(), pda_owner.try_data_len()?) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        // update owner and state
        let mut owner_state = VotingOwner::unpack_unchecked(&pda_owner.data.borrow_mut())?;

        let mut state_state = VotingState::unpack_unchecked(&pda_state.data.borrow_mut())?;

        // CHECK ALREADY INITIALIZED ACCOUNTS
        if owner_state.is_initialized() || state_state.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        // update owner state
        owner_state.discriminant = VOTINGOWNER;
        owner_state.is_initialized = true;
        owner_state.owner = *author.key;
        owner_state.voting_state = *pda_state.key;
        owner_state.bump = bump_owner;

        // update voting state
        state_state.discriminant = VOTINGSTATE;
        state_state.is_initialized = true;
        state_state.voting_owner = *pda_owner.key;

        let clock: Clock = Clock::get()?;

        state_state.voting_started = clock.unix_timestamp;

        match state_state.voting_started.checked_add(7 * 24 * 60 * 60) {
            Some(sucess) => state_state.voting_ends = sucess,
            None => return Err(JanecekError::AdditionOverflow.into()),
        }

        state_state.bump = bump_state;

        VotingOwner::pack(owner_state, &mut &mut pda_owner.data.borrow_mut()[..])?;
        VotingState::pack(state_state, &mut &mut pda_state.data.borrow_mut()[..])?;

        Ok(())
    }

    fn process_create_party(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ix_data: &[u8],
    ) -> ProgramResult {
        let ix = instruction::CreateParty::deserialize(&mut &ix_data[..])
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        let instruction::CreateParty {
            bump_owner,
            bump_state,
            bump_party,
            name,
        } = ix;

        let accounts_iter = &mut accounts.iter();

        let author = next_account_info(accounts_iter)?;

        let owner = next_account_info(accounts_iter)?;

        let pda_owner = next_account_info(accounts_iter)?;

        let pda_state = next_account_info(accounts_iter)?;

        let pda_party = next_account_info(accounts_iter)?;

        let system_program = next_account_info(accounts_iter)?;

        let (pda_owner_, bump_owner_) =
            Pubkey::find_program_address(&[b"voting_owner", owner.key.as_ref()], program_id);

        let (pda_state_, bump_state_) =
            Pubkey::find_program_address(&[b"voting_state", pda_owner_.as_ref()], program_id);

        let (pda_party_, bump_party_) =
            Pubkey::find_program_address(&[name.as_bytes(), pda_state_.as_ref()], program_id);

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
                &[author.clone(), pda_party.clone()],
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
                    &[author.clone(), pda_party.clone()],
                    &[&[&name.as_bytes(), pda_state.key.as_ref(), &[bump_party]]],
                )?;
            }

            program::invoke_signed(
                &system_instruction::allocate(pda_party.key, Party::LEN as u64),
                &[pda_party.clone()],
                &[&[&name.as_bytes(), pda_state.key.as_ref(), &[bump_party]]],
            )?;

            program::invoke_signed(
                &system_instruction::assign(pda_party.key, program_id),
                &[pda_party.clone()],
                &[&[&name.as_bytes(), pda_state.key.as_ref(), &[bump_party]]],
            )?;
        }

        // SIGNER CHECK
        if !owner.is_signer || !author.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // PROGRAM OWNERS CHECK
        if *pda_owner.owner != id() || *pda_state.owner != id() || *pda_party.owner != id() {
            return Err(ProgramError::IllegalOwner);
        }

        // SYSTEM PROGRAM ID
        if *system_program.key != solana_program::system_program::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        // Any account that may be mutated by the program during execution, either its
        // data or metadata such as held lamports, must be writable.
        // MUTABLES CHECK
        if !pda_party.is_writable {
            return Err(JanecekError::AccountNotmutable.into());
        }
        if !author.is_writable {
            return Err(JanecekError::AccountNotmutable.into());
        }
        if system_program.is_writable
            || owner.is_writable
            || pda_owner.is_writable
            || pda_state.is_writable
        {
            return Err(JanecekError::AccountMutable.into());
        }

        // CHECK RENT EXEMPT
        if !rent.is_exempt(pda_party.lamports(), pda_party.try_data_len()?)
            || !rent.is_exempt(pda_owner.lamports(), pda_owner.try_data_len()?)
            || !rent.is_exempt(pda_state.lamports(), pda_state.try_data_len()?)
        {
            return Err(ProgramError::AccountNotRentExempt);
        }

        // deserialize data and check if both are initialized and if owner match
        let owner_state = VotingOwner::unpack(&pda_owner.data.borrow_mut())?;

        let voting_state = VotingState::unpack(&pda_state.data.borrow_mut())?;

        let mut party_state = Party::unpack_unchecked(&pda_party.data.borrow_mut())?;

        // CHECK ALREADY INITIALIZED ACCOUNTS
        if party_state.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        // CHECK NOT YET INITIALIZED ACCOUNTS
        if !owner_state.is_initialized() || !voting_state.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        Self::check_add_context(
            &owner_state,
            &voting_state,
            &pda_owner.key,
            &pda_state.key,
            &owner.key,
        )?;

        // check if voting ended
        let clock: Clock = Clock::get()?;
        if clock.unix_timestamp > voting_state.voting_ends {
            return Err(JanecekError::VotingEnded.into());
        }

        // this probably should not happen as pda will fall if seed have length
        // longer than 32 bytes, but double check the length of name.
        // later in functions, unpack checks name length
        if name.chars().count() > NAME_LENGTH{
            return Err(ProgramError::InvalidInstructionData);
        }

        // create party state
        party_state.discriminant = PARTY;
        party_state.is_initialized = true;
        party_state.author = *author.key;
        party_state.voting_state = *pda_state.key;
        party_state.created = clock.unix_timestamp;
        party_state.name = name;
        party_state.votes = 0;
        party_state.bump = bump_party;

        // pda correctness
        if pda_party_ != *pda_party.key
            || bump_party_ != bump_party
            || party_state.bump != bump_party
        {
            return Err(ProgramError::InvalidSeeds);
        }
        if pda_owner_ != *pda_owner.key
            || bump_owner_ != bump_owner
            || owner_state.bump != bump_owner
        {
            return Err(ProgramError::InvalidSeeds);
        }
        if pda_state_ != *pda_state.key
            || bump_state_ != bump_state
            || voting_state.bump != bump_state
        {
            return Err(ProgramError::InvalidSeeds);
        }

        Party::pack(party_state, &mut &mut pda_party.data.borrow_mut()[..])?;

        Ok(())
    }

    fn process_create_voter(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ix_data: &[u8],
    ) -> ProgramResult {
        let ix = instruction::CreateVoter::deserialize(&mut &ix_data[..])
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        let instruction::CreateVoter {
            bump_owner,
            bump_state,
            bump_voter,
        } = ix;

        let accounts_iter = &mut accounts.iter();

        let author = next_account_info(accounts_iter)?;

        let owner = next_account_info(accounts_iter)?;

        let pda_owner = next_account_info(accounts_iter)?;

        let pda_state = next_account_info(accounts_iter)?;

        let pda_voter = next_account_info(accounts_iter)?;

        let system_program = next_account_info(accounts_iter)?;

        let (pda_owner_, bump_owner_) =
            Pubkey::find_program_address(&[b"voting_owner", owner.key.as_ref()], program_id);

        let (pda_state_, bump_state_) =
            Pubkey::find_program_address(&[b"voting_state", pda_owner_.as_ref()], program_id);

        let (pda_voter_, bump_voter_) = Pubkey::find_program_address(
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
                &[author.clone(), pda_voter.clone()],
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
                    &[author.clone(), pda_voter.clone()],
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
                &[pda_voter.clone()],
                &[&[
                    b"new_voter".as_ref(),
                    author.key.as_ref(),
                    pda_state.key.as_ref(),
                    &[bump_voter],
                ]],
            )?;

            program::invoke_signed(
                &system_instruction::assign(pda_voter.key, program_id),
                &[pda_voter.clone()],
                &[&[
                    b"new_voter".as_ref(),
                    author.key.as_ref(),
                    pda_state.key.as_ref(),
                    &[bump_voter],
                ]],
            )?;
        }

        // SIGNER CHECK
        if !author.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        // PROGRAM OWNERS CHECK
        if *pda_owner.owner != id() || *pda_state.owner != id() || *pda_voter.owner != id() {
            return Err(ProgramError::IllegalOwner);
        }

        // SYSTEM PROGRAM ID
        if *system_program.key != solana_program::system_program::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        // MUTABLES CHECK
        if !pda_voter.is_writable {
            return Err(JanecekError::AccountNotmutable.into());
        }
        if !author.is_writable {
            return Err(JanecekError::AccountNotmutable.into());
        }
        if system_program.is_writable
            || owner.is_writable
            || pda_owner.is_writable
            || pda_state.is_writable
        {
            return Err(JanecekError::AccountMutable.into());
        }

        // CHECK RENT EXEMPT
        if !rent.is_exempt(pda_voter.lamports(), pda_voter.try_data_len()?)
            || !rent.is_exempt(pda_owner.lamports(), pda_owner.try_data_len()?)
            || !rent.is_exempt(pda_state.lamports(), pda_state.try_data_len()?)
        {
            return Err(ProgramError::AccountNotRentExempt);
        }

        // deserialize data and check if both are initialized and if owner match
        let owner_state = VotingOwner::unpack(&pda_owner.data.borrow_mut())?;

        let voting_state = VotingState::unpack(&pda_state.data.borrow_mut())?;

        let mut voter_state = Voter::unpack_unchecked(&pda_voter.data.borrow_mut())?;

        // CHECK ALREADY INITIALIZED ACCOUNTS
        if voter_state.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        // CHECK NOT YET INITIALIZED ACCOUNTS
        if !owner_state.is_initialized() || !voting_state.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        Self::check_add_context(
            &owner_state,
            &voting_state,
            &pda_owner.key,
            &pda_state.key,
            &owner.key,
        )?;

        Self::check_voting_end(&voting_state)?;

        // update voter data
        voter_state.discriminant = VOTER;
        voter_state.is_initialized = true;
        voter_state.author = *author.key;
        voter_state.voting_state = *pda_state.key;
        voter_state.num_votes = 3;
        voter_state.bump = bump_voter;

        // pda correctness
        if pda_voter_ != *pda_voter.key
            || bump_voter_ != bump_voter
            || voter_state.bump != bump_voter
        {
            return Err(ProgramError::InvalidSeeds);
        }
        if pda_owner_ != *pda_owner.key
            || bump_owner_ != bump_owner
            || owner_state.bump != bump_owner
        {
            return Err(ProgramError::InvalidSeeds);
        }
        if pda_state_ != *pda_state.key
            || bump_state_ != bump_state
            || voting_state.bump != bump_state
        {
            return Err(ProgramError::InvalidSeeds);
        }

        Voter::pack(voter_state, &mut &mut pda_voter.data.borrow_mut()[..])?;

        Ok(())
    }
    fn process_vote_positive(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ix_data: &[u8],
    ) -> ProgramResult {
        let ix = instruction::Vote::deserialize(&mut &ix_data[..])
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        let accounts_iter = &mut accounts.iter();

        let _author = next_account_info(accounts_iter)?;

        let _owner = next_account_info(accounts_iter)?;

        let _pda_owner = next_account_info(accounts_iter)?;

        let _pda_state = next_account_info(accounts_iter)?;

        let pda_voter = next_account_info(accounts_iter)?;

        let pda_party = next_account_info(accounts_iter)?;

        Self::check_vote_accounts(&program_id, &accounts, &ix)?;

        let mut voter_state = Voter::unpack(&pda_voter.data.borrow_mut())?;

        match voter_state.num_votes {
            0 => {
                return Err(JanecekError::NoMoreVotes.into());
            }
            1 => {
                return Err(JanecekError::NoMorePosVotes.into());
            }
            2 => {
                let mut party_state = Party::unpack(&pda_party.data.borrow_mut())?;
                if voter_state.pos1 == *pda_party.key {
                    return Err(JanecekError::NoBothPosSameParty.into());
                } else {
                    match voter_state.num_votes.checked_sub(1) {
                        Some(sucess) => voter_state.num_votes = sucess,
                        None => return Err(JanecekError::SubtractionOverflow.into()),
                    }
                    voter_state.pos2 = *pda_party.key;

                    match party_state.votes.checked_add(1) {
                        Some(sucess) => party_state.votes = sucess,
                        None => return Err(JanecekError::AdditionOverflow.into()),
                    }

                    Voter::pack(voter_state, &mut &mut pda_voter.data.borrow_mut()[..])?;
                    Party::pack(party_state, &mut &mut pda_party.data.borrow_mut()[..])?;

                    Ok(())
                }
            }
            3 => {
                let mut party_state = Party::unpack(&pda_party.data.borrow_mut())?;
                match voter_state.num_votes.checked_sub(1) {
                    Some(sucess) => voter_state.num_votes = sucess,
                    None => return Err(JanecekError::SubtractionOverflow.into()),
                }
                voter_state.pos1 = *pda_party.key;
                match party_state.votes.checked_add(1) {
                    Some(sucess) => party_state.votes = sucess,
                    None => return Err(JanecekError::AdditionOverflow.into()),
                }
                Voter::pack(voter_state, &mut &mut pda_voter.data.borrow_mut()[..])?;
                Party::pack(party_state, &mut &mut pda_party.data.borrow_mut()[..])?;
                Ok(())
            }
            _ => {
                return Err(JanecekError::VotesOutOfRange.into());
            }
        }
    }

    fn process_vote_negative(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ix_data: &[u8],
    ) -> ProgramResult {
        let ix = instruction::Vote::deserialize(&mut &ix_data[..])
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        let accounts_iter = &mut accounts.iter();

        let _author = next_account_info(accounts_iter)?;

        let _owner = next_account_info(accounts_iter)?;

        let _pda_owner = next_account_info(accounts_iter)?;

        let _pda_state = next_account_info(accounts_iter)?;

        let pda_voter = next_account_info(accounts_iter)?;

        let pda_party = next_account_info(accounts_iter)?;

        Self::check_vote_accounts(&program_id, &accounts, &ix)?;

        let mut voter_state = Voter::unpack(&pda_voter.data.borrow_mut())?;

        match voter_state.num_votes {
            0 => {
                return Err(JanecekError::NoMoreVotes.into());
            }
            1 => {
                let mut party_state = Party::unpack(&pda_party.data.borrow_mut())?;
                match voter_state.num_votes.checked_sub(1) {
                    Some(sucess) => voter_state.num_votes = sucess,
                    None => return Err(JanecekError::SubtractionOverflow.into()),
                }
                voter_state.neg1 = *pda_party.key;

                match party_state.votes.checked_sub(1) {
                    Some(sucess) => party_state.votes = sucess,
                    None => return Err(JanecekError::SubtractionOverflow.into()),
                }

                Voter::pack(voter_state, &mut &mut pda_voter.data.borrow_mut()[..])?;
                Party::pack(party_state, &mut &mut pda_party.data.borrow_mut()[..])?;

                Ok(())
            }
            2 => {
                return Err(JanecekError::VoteNegativeConstrain.into());
            }
            3 => {
                return Err(JanecekError::VoteNegativeConstrain.into());
            }
            _ => {
                return Err(JanecekError::VotesOutOfRange.into());
            }
        }
    }
    fn check_voting_end(voting_state: &VotingState) -> ProgramResult {
        let clock: Clock = Clock::get().unwrap();
        if clock.unix_timestamp > voting_state.voting_ends {
            return Err(JanecekError::VotingEnded.into());
        }
        Ok(())
    }
    fn check_vote_accounts(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ix_vote: &instruction::Vote,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let author = next_account_info(accounts_iter)?;

        let owner = next_account_info(accounts_iter)?;

        let pda_owner = next_account_info(accounts_iter)?;

        let pda_state = next_account_info(accounts_iter)?;

        let pda_voter = next_account_info(accounts_iter)?;

        let pda_party = next_account_info(accounts_iter)?;

        let (pda_owner_, bump_owner_) =
            Pubkey::find_program_address(&[b"voting_owner", owner.key.as_ref()], program_id);

        let (pda_state_, bump_state_) =
            Pubkey::find_program_address(&[b"voting_state", pda_owner_.as_ref()], program_id);

        let (pda_voter_, bump_voter_) = Pubkey::find_program_address(
            &[b"new_voter", author.key.as_ref(), pda_state_.as_ref()],
            program_id,
        );
        let (pda_party_, bump_party_) = Pubkey::find_program_address(
            &[ix_vote.name.as_bytes(), pda_state_.as_ref()],
            program_id,
        );

        let rent = Rent::get()?;
        if !rent.is_exempt(pda_voter.lamports(), pda_voter.try_data_len()?)
            || !rent.is_exempt(pda_owner.lamports(), pda_owner.try_data_len()?)
            || !rent.is_exempt(pda_state.lamports(), pda_state.try_data_len()?)
            || !rent.is_exempt(pda_party.lamports(), pda_party.try_data_len()?)
        {
            return Err(ProgramError::AccountNotRentExempt);
        }

        let voter_state = Voter::unpack(&pda_voter.data.borrow_mut())?;

        let party_state = Party::unpack(&pda_party.data.borrow_mut())?;

        let owner_state = VotingOwner::unpack(&pda_owner.data.borrow_mut())?;

        let voting_state = VotingState::unpack(&pda_state.data.borrow_mut())?;

        // SIGNER CHECK
        if !author.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        // PROGRAM OWNERS CHECK
        if *pda_owner.owner != id()
            || *pda_state.owner != id()
            || *pda_party.owner != id()
            || *pda_voter.owner != id()
        {
            return Err(ProgramError::IllegalOwner);
        }

        // MUTABLES CHECK
        if !pda_voter.is_writable {
            return Err(JanecekError::AccountNotmutable.into());
        }
        if !pda_party.is_writable {
            return Err(JanecekError::AccountNotmutable.into());
        }
        if !author.is_writable {
            return Err(JanecekError::AccountNotmutable.into());
        }
        if owner.is_writable || pda_owner.is_writable || pda_state.is_writable {
            return Err(JanecekError::AccountMutable.into());
        }

        // check PDA correctness
        if pda_owner_ != *pda_owner.key
            || bump_owner_ != ix_vote.bump_owner
            || owner_state.bump != ix_vote.bump_owner
        {
            return Err(ProgramError::InvalidSeeds);
        }
        if pda_state_ != *pda_state.key
            || bump_state_ != ix_vote.bump_state
            || voting_state.bump != ix_vote.bump_state
        {
            return Err(ProgramError::InvalidSeeds);
        }
        if pda_voter_ != *pda_voter.key
            || bump_voter_ != ix_vote.bump_voter
            || voter_state.bump != ix_vote.bump_voter
        {
            return Err(ProgramError::InvalidSeeds);
        }
        if pda_party_ != *pda_party.key
            || bump_party_ != ix_vote.bump_party
            || party_state.bump != ix_vote.bump_party
        {
            return Err(ProgramError::InvalidSeeds);
        }
        Self::check_vote_context(
            &owner_state,
            &voting_state,
            &party_state,
            &voter_state,
            &author.key,
            &owner.key,
            &pda_owner.key,
            &pda_state.key,
        )?;

        Self::check_voting_end(&voting_state)?;

        Ok(())
    }
    fn check_add_context(
        owner_state: &VotingOwner,
        voting_state: &VotingState,
        pda_owner: &Pubkey,
        pda_state: &Pubkey,
        owner: &Pubkey,
    ) -> ProgramResult {
        // double check voting owner and initiator
        if *owner != owner_state.owner {
            return Err(JanecekError::VotingOwnerMismatch.into());
        }

        // double check voting owner and initiator
        if voting_state.voting_owner != *pda_owner {
            return Err(JanecekError::VotingOwnerMismatch.into());
        }

        // check if state corresponds
        if owner_state.voting_state != *pda_state {
            return Err(JanecekError::VotingStateMismatch.into());
        }
        Ok(())
    }
    fn check_vote_context(
        owner_state: &VotingOwner,
        voting_state: &VotingState,
        party_state: &Party,
        voter_state: &Voter,
        author: &Pubkey,
        owner: &Pubkey,
        pda_owner: &Pubkey,
        pda_state: &Pubkey,
    ) -> ProgramResult {
        // CHECK NOT YET INITIALIZED ACCOUNTS (unpack checks this, but why not double check)
        if !owner_state.is_initialized()
            || !voting_state.is_initialized()
            || !party_state.is_initialized()
            || !voter_state.is_initialized()
        {
            return Err(ProgramError::UninitializedAccount);
        }

        // check if author corresponds to voter
        if *author != voter_state.author {
            return Err(JanecekError::VoterMismatch.into());
        }

        // check if owner corresponds to the voting owner
        if *owner != owner_state.owner {
            return Err(JanecekError::VotingOwnerMismatch.into());
        }

        // check if voting state corresponds to the voting owner
        if voting_state.voting_owner != *pda_owner {
            return Err(JanecekError::VotingOwnerMismatch.into());
        }
        // // check if voting owner corresponds to the voting state
        if owner_state.voting_state != *pda_state {
            return Err(JanecekError::VotingStateMismatch.into());
        }
        // check if voter correspoinds to this voting state
        if *pda_state != voter_state.voting_state {
            return Err(JanecekError::VotingStateMismatch.into());
        }
        // check if party correspoinds to this voting state
        if *pda_state != party_state.voting_state {
            return Err(JanecekError::VotingStateMismatch.into());
        }
        Ok(())
    }
}
