use crate::entrypoint::id;
use crate::error::JanecekError;
use crate::instruction::{
    get_owner_address, get_party_address, get_state_address, get_voter_address, JanecekInstruction,
    VotePreference,
};
use crate::state::JanecekState;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    program,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::state::VotesStates;

pub fn entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != id() {
        return Err(ProgramError::IncorrectProgramId);
    }
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }
    dispatch(program_id, accounts, instruction_data)
}
fn dispatch(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let mut ix_data: &[u8] = data;

    match JanecekInstruction::deserialize(&mut ix_data)? {
        JanecekInstruction::Initialize => process_initialize(program_id, accounts),
        JanecekInstruction::CreateParty {
            bump_owner,
            bump_state,
            name,
        } => process_create_party(program_id, accounts, &[bump_owner, bump_state], name),
        JanecekInstruction::CreateVoter {
            bump_owner,
            bump_state,
        } => process_create_voter(program_id, accounts, &[bump_owner, bump_state]),
        JanecekInstruction::VoteNeg {
            bump_owner,
            bump_state,
            bump_voter,
            bump_party,
            name,
        } => process_vote(
            program_id,
            accounts,
            &[bump_owner, bump_state, bump_voter, bump_party],
            VotePreference::Negative,
            name,
        ),
        JanecekInstruction::VotePos {
            bump_owner,
            bump_state,
            bump_voter,
            bump_party,
            name,
        } => process_vote(
            program_id,
            accounts,
            &[bump_owner, bump_state, bump_voter, bump_party],
            VotePreference::Positive,
            name,
        ),
    }
}

/// check if account is Signer
fn try_signer(account: &AccountInfo) -> ProgramResult {
    if !account.is_signer {
        Err(ProgramError::MissingRequiredSignature)
    } else {
        Ok(())
    }
}

// fn try_writable(account: &AccountInfo) -> ProgramResult {
//     if !account.is_writable {
//         Err(JanecekError::AccountNotmutable.into())
//     } else {
//         Ok(())
//     }
// }

/// check if account owner is Program ID
fn try_owner(account: &AccountInfo) -> ProgramResult {
    if *account.owner != id() {
        Err(ProgramError::IllegalOwner)
    } else {
        Ok(())
    }
}

fn try_system_owner(account: &AccountInfo) -> ProgramResult {
    if *account.owner != solana_program::system_program::id() {
        Err(ProgramError::IllegalOwner)
    } else {
        Ok(())
    }
}

/// check if account is rent exempt
fn try_rent_exempt(account: &AccountInfo) -> ProgramResult {
    let rent = Rent::get()?;
    if !rent.is_exempt(account.lamports(), account.try_data_len()?) {
        Err(ProgramError::AccountNotRentExempt)
    } else {
        Ok(())
    }
}

/// check if provided PDA equals to derived PDA
fn try_seeds(provided: &Pubkey, derived: &Pubkey) -> ProgramResult {
    if provided != derived {
        Err(ProgramError::InvalidSeeds)
    } else {
        Ok(())
    }
}

/// check that provided and derived bumps are equal
fn try_bumps(provided: u8, derived: u8, saved: u8) -> ProgramResult {
    if provided != derived || saved != derived {
        Err(ProgramError::InvalidSeeds)
    } else {
        Ok(())
    }
}

fn try_string_length(string: &String) -> ProgramResult {
    if string.len() > 32 {
        Err(ProgramError::InvalidInstructionData)
    } else {
        Ok(())
    }
}

/// check if account Pubkey is equal to System Program
fn try_system_program(account: &AccountInfo) -> ProgramResult {
    if *account.key != solana_program::system_program::id() {
        Err(ProgramError::IncorrectProgramId)
    } else {
        Ok(())
    }
}

/// check if voting ended for given timestamp
fn try_voting_ended(deadline: i64, timestamp: i64) -> ProgramResult {
    if timestamp > deadline {
        Err(JanecekError::VotingEnded.into())
    } else {
        Ok(())
    }
}

/// check if account already initialized
fn try_initialized(is_initialized: bool) -> ProgramResult {
    if is_initialized {
        Err(ProgramError::AccountAlreadyInitialized)
    } else {
        Ok(())
    }
}

/// check if account not initialized yet
fn try_uninitialized(is_initialized: bool) -> ProgramResult {
    if !is_initialized {
        Err(ProgramError::UninitializedAccount)
    } else {
        Ok(())
    }
}

/// check if voting owner`s author and author provided are same
fn try_author(author: &AccountInfo, data_author: &Pubkey) -> ProgramResult {
    if *author.key != *data_author {
        Err(ProgramError::IllegalOwner)
    } else {
        Ok(())
    }
}

fn try_voting_state(pda_state: &AccountInfo, voting_state: &Pubkey) -> ProgramResult {
    if *pda_state.key != *voting_state {
        Err(JanecekError::VotingStateMismatch.into())
    } else {
        Ok(())
    }
}

fn try_voting_owner(pda_owner: &AccountInfo, voting_owner: &Pubkey) -> ProgramResult {
    if *pda_owner.key != *voting_owner {
        Err(JanecekError::VotingOwnerMismatch.into())
    } else {
        Ok(())
    }
}

fn try_vote_positive(
    num_votes: &mut VotesStates,
    pos1: &mut Pubkey,
    pos2: &mut Pubkey,
    pda_party: &AccountInfo,
) -> ProgramResult {
    match num_votes {
        VotesStates::Full => {
            *num_votes = VotesStates::OneSpent;
            *pos1 = *pda_party.key;
            Ok(())
        }
        VotesStates::OneSpent => {
            if pos1 == pos2 {
                Err(JanecekError::NoBothPosSameParty.into())
            } else {
                *num_votes = VotesStates::NoMorePositiveVotes;
                *pos2 = *pda_party.key;
                Ok(())
            }
        }
        VotesStates::NoMorePositiveVotes => Err(JanecekError::NoMorePosVotes.into()),
        VotesStates::NoMoreVotes => Err(JanecekError::NoMoreVotes.into()),
    }
}

fn try_vote_negative(
    num_votes: &mut VotesStates,
    neg1: &mut Pubkey,
    pda_party: &AccountInfo,
) -> ProgramResult {
    match num_votes {
        VotesStates::Full => Err(JanecekError::VoteNegativeConstrain.into()),
        VotesStates::OneSpent => Err(JanecekError::VoteNegativeConstrain.into()),
        VotesStates::NoMorePositiveVotes => {
            *neg1 = *pda_party.key;
            *num_votes = VotesStates::NoMoreVotes;
            Ok(())
        }
        VotesStates::NoMoreVotes => Err(JanecekError::NoMoreVotes.into()),
    }
}

fn try_increase_votes(votes: &mut i64) -> ProgramResult {
    match votes.checked_add(1) {
        Some(sucess) => {
            *votes = sucess;
            Ok(())
        }
        None => Err(JanecekError::AdditionOverflow.into()),
    }
}
fn try_decrease_votes(votes: &mut i64) -> ProgramResult {
    match votes.checked_sub(1) {
        Some(sucess) => {
            *votes = sucess;
            Ok(())
        }
        None => Err(JanecekError::AdditionOverflow.into()),
    }
}
/// create_account or transfer/allocate/assign
fn try_create_or_assign(
    from_account: &AccountInfo,
    to_account: &AccountInfo,
    data_len: usize,
    account_infos: &[AccountInfo],
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let rent = Rent::get()?;
    let current_lamports = to_account.lamports();
    let lamports_needed = rent.minimum_balance(data_len);

    if current_lamports == 0 {
        program::invoke_signed(
            &system_instruction::create_account(
                from_account.key,
                to_account.key,
                lamports_needed,
                data_len as u64,
                &id(),
            ),
            account_infos,
            signers_seeds,
        )?;
    } else {
        let required_lamports = rent
            .minimum_balance(data_len)
            .max(1)
            .saturating_sub(current_lamports);

        if required_lamports > 0 {
            program::invoke_signed(
                &system_instruction::transfer(from_account.key, to_account.key, required_lamports),
                account_infos,
                signers_seeds,
            )?;
        }

        program::invoke_signed(
            &system_instruction::allocate(to_account.key, data_len as u64),
            account_infos,
            signers_seeds,
        )?;

        program::invoke_signed(
            &system_instruction::assign(to_account.key, &id()),
            account_infos,
            signers_seeds,
        )?;
    }
    Ok(())
}

fn process_initialize(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let author = next_account_info(accounts_iter)?;
    try_signer(author)?;
    try_system_owner(author)?;

    let pda_owner = next_account_info(accounts_iter)?;
    let (pda_owner_, bump_owner) = get_owner_address(*author.key);
    try_seeds(&pda_owner_, pda_owner.key)?;
    try_system_owner(pda_owner)?;

    let pda_state = next_account_info(accounts_iter)?;
    let (pda_state_, bump_state) = get_state_address(pda_owner_);
    try_seeds(&pda_state_, pda_state.key)?;
    try_system_owner(pda_state)?;

    let system_program = next_account_info(accounts_iter)?;
    try_system_program(system_program)?;

    try_create_or_assign(
        author,
        pda_owner,
        JanecekState::LEN_VOTINGOWNER,
        &[author.clone(), pda_owner.clone()],
        &[&[b"voting_owner".as_ref(), author.key.as_ref(), &[bump_owner]]],
    )?;
    try_owner(pda_owner)?;
    try_rent_exempt(pda_owner)?;

    try_create_or_assign(
        author,
        pda_state,
        JanecekState::LEN_VOTINGSTATE,
        &[author.clone(), pda_state.clone()],
        &[&[
            b"voting_state".as_ref(),
            pda_owner.key.as_ref(),
            &[bump_state],
        ]],
    )?;
    try_owner(pda_state)?;
    try_rent_exempt(pda_state)?;

    match JanecekState::deserialize(&mut &(*pda_owner.data).borrow_mut()[..])? {
        JanecekState::Fresh => {}
        JanecekState::VotingOwner { is_initialized, .. } => {
            try_initialized(is_initialized)?;
        }
        _ => return Err(ProgramError::InvalidAccountData),
    };
    let owner = JanecekState::VotingOwner {
        is_initialized: true,
        author: *author.key,
        voting_state: *pda_state.key,
        bump: bump_owner,
    };
    owner
        .serialize(&mut &mut (*pda_owner.data).borrow_mut()[..])
        .unwrap();

    // Is this OK ???
    match JanecekState::deserialize(&mut &(*pda_state.data).borrow_mut()[..])? {
        JanecekState::Fresh => {}
        JanecekState::VotingState { is_initialized, .. } => {
            try_initialized(is_initialized)?;
        }
        _ => return Err(ProgramError::InvalidAccountData),
    };

    let clock: Clock = Clock::get()?;
    let start = clock.unix_timestamp;

    // add 7 days to the current time, as voting lasts for 7 days
    let end = match start.checked_add(7 * 24 * 60 * 60) {
        Some(sucess) => sucess,
        None => return Err(JanecekError::AdditionOverflow.into()),
    };
    let state = JanecekState::VotingState {
        is_initialized: true,
        voting_owner: *pda_owner.key,
        voting_started: start,
        voting_ends: end,
        bump: bump_state,
    };

    state
        .serialize(&mut &mut (*pda_state.data).borrow_mut()[..])
        .unwrap();
    Ok(())
}

fn process_create_party(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    bumps: &[u8],
    name: String,
) -> ProgramResult {
    try_string_length(&name)?;

    let bump_iter = &mut bumps.iter();

    let bump_owner_provided: &u8 = bump_iter.next().unwrap();
    let bump_state_provided: &u8 = bump_iter.next().unwrap();

    let accounts_iter = &mut accounts.iter();

    // this is party author
    let author = next_account_info(accounts_iter)?;
    try_signer(author)?;
    try_system_owner(author)?;

    // bad name but, owner here is owner of voting, so acc that called initialize
    let owner = next_account_info(accounts_iter)?;
    try_signer(owner)?;
    try_system_owner(owner)?;

    let pda_owner = next_account_info(accounts_iter)?;
    let (pda_owner_, bump_owner) = get_owner_address(*owner.key);
    try_seeds(&pda_owner_, pda_owner.key)?;
    try_owner(pda_owner)?;
    try_rent_exempt(pda_owner)?;

    let pda_state = next_account_info(accounts_iter)?;
    let (pda_state_, bump_state) = get_state_address(pda_owner_);
    try_seeds(&pda_state_, pda_state.key)?;
    try_owner(pda_state)?;
    try_rent_exempt(pda_state)?;

    let pda_party = next_account_info(accounts_iter)?;
    let (pda_party_, bump_party) = get_party_address(&name, pda_state_);
    try_seeds(&pda_party_, pda_party.key)?;
    try_system_owner(pda_party)?;

    let system_program = next_account_info(accounts_iter)?;
    try_system_program(system_program)?;

    try_create_or_assign(
        author,
        pda_party,
        JanecekState::LEN_PARTY,
        &[author.clone(), pda_party.clone()],
        &[&[(name.as_bytes()), pda_state.key.as_ref(), &[bump_party]]],
    )?;
    try_owner(pda_party)?;
    try_rent_exempt(pda_party)?;

    match JanecekState::deserialize(&mut &(*pda_owner.data).borrow_mut()[..])? {
        JanecekState::VotingOwner {
            is_initialized,
            author,
            voting_state,
            bump,
        } => {
            try_uninitialized(is_initialized)?;
            try_author(owner, &author)?;
            try_voting_state(pda_state, &voting_state)?;
            try_bumps(*bump_owner_provided, bump_owner, bump)?;
        }
        _ => return Err(ProgramError::InvalidAccountData),
    };
    let now = match JanecekState::deserialize(&mut &(*pda_state.data).borrow_mut()[..])? {
        JanecekState::VotingState {
            is_initialized,
            voting_owner,
            voting_started: _,
            voting_ends,
            bump,
        } => {
            try_uninitialized(is_initialized)?;
            try_voting_owner(pda_owner, &voting_owner)?;
            try_bumps(*bump_state_provided, bump_state, bump)?;
            let clock: Clock = Clock::get()?;
            let now = clock.unix_timestamp;
            try_voting_ended(voting_ends, now)?;
            now
        }
        _ => return Err(ProgramError::InvalidAccountData),
    };

    match JanecekState::deserialize(&mut &(*pda_party.data).borrow_mut()[..])? {
        JanecekState::Fresh => {}
        JanecekState::Party { is_initialized, .. } => {
            try_initialized(is_initialized)?;
        }
        _ => return Err(ProgramError::InvalidAccountData),
    };

    let mut name_bytearray: [u8; JanecekState::NAME_LENGTH] = [0u8; JanecekState::NAME_LENGTH];

    name_bytearray[..name.len()].copy_from_slice(name.into_bytes().as_slice());

    let party = JanecekState::Party {
        is_initialized: true,
        author: *author.key,
        voting_state: *pda_state.key,
        created: now,
        name: name_bytearray,
        votes: 0,
        bump: bump_party,
    };

    party
        .serialize(&mut &mut (*pda_party.data).borrow_mut()[..])
        .unwrap();
    Ok(())
}

fn process_create_voter(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    bumps: &[u8],
) -> ProgramResult {
    let bump_iter = &mut bumps.iter();
    let bump_owner_provided: &u8 = bump_iter.next().unwrap();
    let bump_state_provided: &u8 = bump_iter.next().unwrap();

    let accounts_iter = &mut accounts.iter();

    let author = next_account_info(accounts_iter)?;
    try_signer(author)?;
    try_system_owner(author)?;

    let owner = next_account_info(accounts_iter)?;
    try_system_owner(owner)?;

    let pda_owner = next_account_info(accounts_iter)?;
    let (pda_owner_, bump_owner_) = get_owner_address(*owner.key);
    try_owner(pda_owner)?;
    try_seeds(&pda_owner_, pda_owner.key)?;
    try_rent_exempt(pda_owner)?;

    let pda_state = next_account_info(accounts_iter)?;
    let (pda_state_, bump_state_) = get_state_address(pda_owner_);
    try_owner(pda_state)?;
    try_seeds(&pda_state_, pda_state.key)?;
    try_rent_exempt(pda_state)?;

    let pda_voter = next_account_info(accounts_iter)?;
    let (pda_voter_, bump_voter) = get_voter_address(*author.key, pda_state_);
    try_system_owner(pda_voter)?;
    try_seeds(&pda_voter_, pda_voter.key)?;

    let system_program = next_account_info(accounts_iter)?;
    try_system_program(system_program)?;

    try_create_or_assign(
        author,
        pda_voter,
        JanecekState::LEN_VOTER,
        &[author.clone(), pda_voter.clone()],
        &[&[
            b"new_voter".as_ref(),
            author.key.as_ref(),
            pda_state.key.as_ref(),
            &[bump_voter],
        ]],
    )?;
    try_owner(pda_voter)?;
    try_rent_exempt(pda_voter)?;

    match JanecekState::deserialize(&mut &(*pda_owner.data).borrow_mut()[..])? {
        JanecekState::VotingOwner {
            is_initialized,
            author,
            voting_state,
            bump,
        } => {
            try_uninitialized(is_initialized)?;
            try_author(owner, &author)?;
            try_voting_state(pda_state, &voting_state)?;
            try_bumps(*bump_owner_provided, bump_owner_, bump)?;
        }
        _ => return Err(ProgramError::InvalidAccountData),
    };
    match JanecekState::deserialize(&mut &(*pda_state.data).borrow_mut()[..])? {
        JanecekState::VotingState {
            is_initialized,
            voting_owner,
            voting_started: _,
            voting_ends,
            bump,
        } => {
            try_uninitialized(is_initialized)?;
            try_voting_owner(pda_owner, &voting_owner)?;
            try_bumps(*bump_state_provided, bump_state_, bump)?;
            let clock: Clock = Clock::get()?;
            let now = clock.unix_timestamp;
            try_voting_ended(voting_ends, now)?;
        }
        _ => return Err(ProgramError::InvalidAccountData),
    };
    match JanecekState::deserialize(&mut &(*pda_voter.data).borrow_mut()[..])? {
        JanecekState::Fresh => {}
        JanecekState::Voter { is_initialized, .. } => {
            try_initialized(is_initialized)?;
        }
        _ => return Err(ProgramError::InvalidAccountData),
    };
    let voter = JanecekState::Voter {
        is_initialized: true,
        author: *author.key,
        voting_state: *pda_state.key,
        num_votes: VotesStates::Full,
        pos1: solana_program::system_program::id(),
        pos2: solana_program::system_program::id(),
        neg1: solana_program::system_program::id(),
        bump: bump_voter,
    };

    voter
        .serialize(&mut &mut (*pda_voter.data).borrow_mut()[..])
        .unwrap();
    Ok(())
}
fn process_vote(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    bumps: &[u8],
    vote_preference: VotePreference,
    name: String,
) -> ProgramResult {
    try_string_length(&name)?;
    let bump_iter = &mut bumps.iter();

    let bump_owner_provided: &u8 = bump_iter.next().unwrap();
    let bump_state_provided: &u8 = bump_iter.next().unwrap();
    let bump_voter_provided: &u8 = bump_iter.next().unwrap();
    let bump_party_provided: &u8 = bump_iter.next().unwrap();

    let accounts_iter = &mut accounts.iter();

    let voter_author = next_account_info(accounts_iter)?;
    try_signer(voter_author)?;

    let owner = next_account_info(accounts_iter)?;

    let pda_owner = next_account_info(accounts_iter)?;
    let (pda_owner_, bump_owner_) = get_owner_address(*owner.key);
    try_owner(pda_owner)?;
    try_seeds(&pda_owner_, pda_owner.key)?;
    try_rent_exempt(pda_owner)?;

    let pda_state = next_account_info(accounts_iter)?;
    let (pda_state_, bump_state_) = get_state_address(pda_owner_);
    try_owner(pda_state)?;
    try_seeds(&pda_state_, pda_state.key)?;
    try_rent_exempt(pda_state)?;

    let pda_voter = next_account_info(accounts_iter)?;
    let (pda_voter_, bump_voter_) = get_voter_address(*voter_author.key, pda_state_);
    try_owner(pda_voter)?;
    try_seeds(&pda_voter_, pda_voter.key)?;
    try_rent_exempt(pda_voter)?;

    let pda_party = next_account_info(accounts_iter)?;
    let (pda_party_, bump_party_) = get_party_address(&name, pda_state_);
    try_owner(pda_party)?;
    try_seeds(&pda_party_, pda_party.key)?;
    try_rent_exempt(pda_party)?;

    match JanecekState::deserialize(&mut &(*pda_owner.data).borrow_mut()[..])? {
        JanecekState::VotingOwner {
            is_initialized,
            author,
            voting_state,
            bump,
        } => {
            try_uninitialized(is_initialized)?;
            try_author(owner, &author)?;
            try_voting_state(pda_state, &voting_state)?;
            try_bumps(*bump_owner_provided, bump_owner_, bump)?;
        }
        _ => return Err(ProgramError::InvalidAccountData),
    };
    match JanecekState::deserialize(&mut &(*pda_state.data).borrow_mut()[..])? {
        JanecekState::VotingState {
            is_initialized,
            voting_owner,
            voting_started: _,
            voting_ends,
            bump,
        } => {
            try_uninitialized(is_initialized)?;
            try_voting_owner(pda_owner, &voting_owner)?;
            try_bumps(*bump_state_provided, bump_state_, bump)?;
            let clock: Clock = Clock::get()?;
            let now = clock.unix_timestamp;
            try_voting_ended(voting_ends, now)?;
        }
        _ => return Err(ProgramError::InvalidAccountData),
    };
    let voter = match JanecekState::deserialize(&mut &(*pda_voter.data).borrow_mut()[..])? {
        JanecekState::Voter {
            is_initialized,
            author,
            voting_state,
            num_votes,
            pos1,
            pos2,
            neg1,
            bump,
        } => {
            try_uninitialized(is_initialized)?;
            try_author(voter_author, &author)?;
            try_voting_state(pda_state, &voting_state)?;
            try_bumps(*bump_voter_provided, bump_voter_, bump)?;
            let mut new_votes = num_votes;
            let mut new_pos1 = pos1;
            let mut new_pos2 = pos2;
            let mut new_neg1 = neg1;

            match vote_preference {
                VotePreference::Negative => {
                    try_vote_negative(&mut new_votes, &mut new_neg1, pda_party)?;
                }
                VotePreference::Positive => {
                    try_vote_positive(&mut new_votes, &mut new_pos1, &mut new_pos2, pda_party)?;
                }
            }
            JanecekState::Voter {
                is_initialized,
                author,
                voting_state,
                num_votes: new_votes,
                pos1: new_pos1,
                pos2: new_pos2,
                neg1: new_neg1,
                bump,
            }
        }
        _ => return Err(ProgramError::InvalidAccountData),
    };
    voter
        .serialize(&mut &mut (*pda_voter.data).borrow_mut()[..])
        .unwrap();

    let party = match JanecekState::deserialize(&mut &(*pda_party.data).borrow_mut()[..])? {
        JanecekState::Party {
            is_initialized,
            author,
            voting_state,
            created,
            name,
            votes,
            bump,
        } => {
            try_uninitialized(is_initialized)?;
            try_voting_state(pda_state, &voting_state)?;
            try_bumps(*bump_party_provided, bump_party_, bump)?;
            let mut new_votes = votes;

            match vote_preference {
                VotePreference::Negative => {
                    try_decrease_votes(&mut new_votes)?;
                }
                VotePreference::Positive => {
                    try_increase_votes(&mut new_votes)?;
                }
            }
            JanecekState::Party {
                is_initialized,
                author,
                voting_state,
                created,
                name,
                votes: new_votes,
                bump,
            }
        }
        _ => return Err(ProgramError::InvalidAccountData),
    };
    party
        .serialize(&mut &mut (*pda_party.data).borrow_mut()[..])
        .unwrap();
    Ok(())
}
