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
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::Sysvar,
};

use crate::try_functions;

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
            name_bytearray,
        } => process_create_party(
            program_id,
            accounts,
            &[bump_owner, bump_state],
            &name_bytearray,
        ),
        JanecekInstruction::CreateVoter {
            bump_owner,
            bump_state,
        } => process_create_voter(program_id, accounts, &[bump_owner, bump_state]),
        JanecekInstruction::VoteNeg {
            bump_owner,
            bump_state,
            bump_voter,
            bump_party,
            name_bytearray,
        } => process_vote(
            program_id,
            accounts,
            &[bump_owner, bump_state, bump_voter, bump_party],
            VotePreference::Negative,
            &name_bytearray,
        ),
        JanecekInstruction::VotePos {
            bump_owner,
            bump_state,
            bump_voter,
            bump_party,
            name_bytearray,
        } => process_vote(
            program_id,
            accounts,
            &[bump_owner, bump_state, bump_voter, bump_party],
            VotePreference::Positive,
            &name_bytearray,
        ),
    }
}

fn process_initialize(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let author = next_account_info(accounts_iter)?;
    try_functions::try_signer(author)?;
    //try_functions::try_system_owner(author)?;

    let pda_owner = next_account_info(accounts_iter)?;
    let (pda_owner_, bump_owner) = get_owner_address(*author.key);
    try_functions::try_seeds(&pda_owner_, pda_owner.key)?;
    //try_functions::try_system_owner(pda_owner)?;

    let pda_state = next_account_info(accounts_iter)?;
    let (pda_state_, bump_state) = get_state_address(pda_owner_);
    try_functions::try_seeds(&pda_state_, pda_state.key)?;
    //try_functions::try_system_owner(pda_state)?;

    let system_program = next_account_info(accounts_iter)?;
    try_functions::try_system_program(system_program)?;

    try_functions::try_create_or_assign(
        author,
        pda_owner,
        JanecekState::LEN_VOTINGOWNER,
        &[author.clone(), pda_owner.clone()],
        &[&[b"voting_owner".as_ref(), author.key.as_ref(), &[bump_owner]]],
    )?;
    try_functions::try_owner(pda_owner)?;
    try_functions::try_rent_exempt(pda_owner)?;

    try_functions::try_create_or_assign(
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
    try_functions::try_owner(pda_state)?;
    try_functions::try_rent_exempt(pda_state)?;

    match JanecekState::deserialize(&mut &(*pda_owner.data).borrow_mut()[..])? {
        JanecekState::Fresh => {}
        JanecekState::VotingOwner { is_initialized, .. } => {
            try_functions::try_initialized(is_initialized)?;
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
            try_functions::try_initialized(is_initialized)?;
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
    name: &[u8; JanecekState::NAME_LENGTH],
) -> ProgramResult {
    let bump_iter = &mut bumps.iter();

    let bump_owner_provided: &u8 = bump_iter.next().unwrap();
    let bump_state_provided: &u8 = bump_iter.next().unwrap();

    let accounts_iter = &mut accounts.iter();

    // this is party author
    let author = next_account_info(accounts_iter)?;
    try_functions::try_signer(author)?;
    //try_functions::try_system_owner(author)?;

    // bad name but, owner here is owner of voting, so acc that called initialize
    let owner = next_account_info(accounts_iter)?;
    try_functions::try_signer(owner)?;
    //try_functions::try_system_owner(owner)?;

    let pda_owner = next_account_info(accounts_iter)?;
    let (pda_owner_, bump_owner) = get_owner_address(*owner.key);
    try_functions::try_seeds(&pda_owner_, pda_owner.key)?;
    try_functions::try_owner(pda_owner)?;
    try_functions::try_rent_exempt(pda_owner)?;

    let pda_state = next_account_info(accounts_iter)?;
    let (pda_state_, bump_state) = get_state_address(pda_owner_);
    try_functions::try_seeds(&pda_state_, pda_state.key)?;
    try_functions::try_owner(pda_state)?;
    try_functions::try_rent_exempt(pda_state)?;

    let pda_party = next_account_info(accounts_iter)?;
    let (pda_party_, bump_party) = get_party_address(name, pda_state_);
    try_functions::try_seeds(&pda_party_, pda_party.key)?;
    //try_functions::try_system_owner(pda_party)?;

    let system_program = next_account_info(accounts_iter)?;
    try_functions::try_system_program(system_program)?;

    try_functions::try_create_or_assign(
        author,
        pda_party,
        JanecekState::LEN_PARTY,
        &[author.clone(), pda_party.clone()],
        &[&[(name), pda_state.key.as_ref(), &[bump_party]]],
    )?;
    try_functions::try_owner(pda_party)?;
    try_functions::try_rent_exempt(pda_party)?;

    match JanecekState::deserialize(&mut &(*pda_owner.data).borrow_mut()[..])? {
        JanecekState::VotingOwner {
            is_initialized,
            author,
            voting_state,
            bump,
        } => {
            try_functions::try_uninitialized(is_initialized)?;
            try_functions::try_author(owner, &author)?;
            try_functions::try_voting_state(pda_state, &voting_state)?;
            try_functions::try_bumps(*bump_owner_provided, bump_owner, bump)?;
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
            try_functions::try_uninitialized(is_initialized)?;
            try_functions::try_voting_owner(pda_owner, &voting_owner)?;
            try_functions::try_bumps(*bump_state_provided, bump_state, bump)?;
            let clock: Clock = Clock::get()?;
            let now = clock.unix_timestamp;
            try_functions::try_voting_ended(voting_ends, now)?;
            now
        }
        _ => return Err(ProgramError::InvalidAccountData),
    };

    match JanecekState::deserialize(&mut &(*pda_party.data).borrow_mut()[..])? {
        JanecekState::Fresh => {}
        JanecekState::Party { is_initialized, .. } => {
            try_functions::try_initialized(is_initialized)?;
        }
        _ => return Err(ProgramError::InvalidAccountData),
    };

    let party = JanecekState::Party {
        is_initialized: true,
        author: *author.key,
        voting_state: *pda_state.key,
        created: now,
        name: *name,
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
    try_functions::try_signer(author)?;
    //try_functions::try_system_owner(author)?;

    let owner = next_account_info(accounts_iter)?;
    //try_functions::try_system_owner(owner)?;

    let pda_owner = next_account_info(accounts_iter)?;
    let (pda_owner_, bump_owner_) = get_owner_address(*owner.key);
    try_functions::try_owner(pda_owner)?;
    try_functions::try_seeds(&pda_owner_, pda_owner.key)?;
    try_functions::try_rent_exempt(pda_owner)?;

    let pda_state = next_account_info(accounts_iter)?;
    let (pda_state_, bump_state_) = get_state_address(pda_owner_);
    try_functions::try_owner(pda_state)?;
    try_functions::try_seeds(&pda_state_, pda_state.key)?;
    try_functions::try_rent_exempt(pda_state)?;

    let pda_voter = next_account_info(accounts_iter)?;
    let (pda_voter_, bump_voter) = get_voter_address(*author.key, pda_state_);
    //try_functions::try_system_owner(pda_voter)?;
    try_functions::try_seeds(&pda_voter_, pda_voter.key)?;

    let system_program = next_account_info(accounts_iter)?;
    try_functions::try_system_program(system_program)?;

    try_functions::try_create_or_assign(
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
    try_functions::try_owner(pda_voter)?;
    try_functions::try_rent_exempt(pda_voter)?;

    match JanecekState::deserialize(&mut &(*pda_owner.data).borrow_mut()[..])? {
        JanecekState::VotingOwner {
            is_initialized,
            author,
            voting_state,
            bump,
        } => {
            try_functions::try_uninitialized(is_initialized)?;
            try_functions::try_author(owner, &author)?;
            try_functions::try_voting_state(pda_state, &voting_state)?;
            try_functions::try_bumps(*bump_owner_provided, bump_owner_, bump)?;
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
            try_functions::try_uninitialized(is_initialized)?;
            try_functions::try_voting_owner(pda_owner, &voting_owner)?;
            try_functions::try_bumps(*bump_state_provided, bump_state_, bump)?;
            let clock: Clock = Clock::get()?;
            let now = clock.unix_timestamp;
            try_functions::try_voting_ended(voting_ends, now)?;
        }
        _ => return Err(ProgramError::InvalidAccountData),
    };
    match JanecekState::deserialize(&mut &(*pda_voter.data).borrow_mut()[..])? {
        JanecekState::Fresh => {}
        JanecekState::Voter { is_initialized, .. } => {
            try_functions::try_initialized(is_initialized)?;
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
    name: &[u8; JanecekState::NAME_LENGTH],
) -> ProgramResult {
    let bump_iter = &mut bumps.iter();

    let bump_owner_provided: &u8 = bump_iter.next().unwrap();
    let bump_state_provided: &u8 = bump_iter.next().unwrap();
    let bump_voter_provided: &u8 = bump_iter.next().unwrap();
    let bump_party_provided: &u8 = bump_iter.next().unwrap();

    let accounts_iter = &mut accounts.iter();

    let voter_author = next_account_info(accounts_iter)?;
    try_functions::try_signer(voter_author)?;
    //try_functions::try_system_owner(voter_author)?;

    let owner = next_account_info(accounts_iter)?;
    //try_functions::try_system_owner(owner)?;

    let pda_owner = next_account_info(accounts_iter)?;
    let (pda_owner_, bump_owner_) = get_owner_address(*owner.key);
    try_functions::try_owner(pda_owner)?;
    try_functions::try_seeds(&pda_owner_, pda_owner.key)?;
    try_functions::try_rent_exempt(pda_owner)?;

    let pda_state = next_account_info(accounts_iter)?;
    let (pda_state_, bump_state_) = get_state_address(pda_owner_);
    try_functions::try_owner(pda_state)?;
    try_functions::try_seeds(&pda_state_, pda_state.key)?;
    try_functions::try_rent_exempt(pda_state)?;

    let pda_voter = next_account_info(accounts_iter)?;
    let (pda_voter_, bump_voter_) = get_voter_address(*voter_author.key, pda_state_);
    try_functions::try_owner(pda_voter)?;
    try_functions::try_seeds(&pda_voter_, pda_voter.key)?;
    try_functions::try_rent_exempt(pda_voter)?;

    let pda_party = next_account_info(accounts_iter)?;
    let (pda_party_, bump_party_) = get_party_address(name, pda_state_);
    try_functions::try_owner(pda_party)?;
    try_functions::try_seeds(&pda_party_, pda_party.key)?;
    try_functions::try_rent_exempt(pda_party)?;

    match JanecekState::deserialize(&mut &(*pda_owner.data).borrow_mut()[..])? {
        JanecekState::VotingOwner {
            is_initialized,
            author,
            voting_state,
            bump,
        } => {
            try_functions::try_uninitialized(is_initialized)?;
            try_functions::try_author(owner, &author)?;
            try_functions::try_voting_state(pda_state, &voting_state)?;
            try_functions::try_bumps(*bump_owner_provided, bump_owner_, bump)?;
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
            try_functions::try_uninitialized(is_initialized)?;
            try_functions::try_voting_owner(pda_owner, &voting_owner)?;
            try_functions::try_bumps(*bump_state_provided, bump_state_, bump)?;
            let clock: Clock = Clock::get()?;
            let now = clock.unix_timestamp;
            try_functions::try_voting_ended(voting_ends, now)?;
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
            try_functions::try_uninitialized(is_initialized)?;
            try_functions::try_author(voter_author, &author)?;
            try_functions::try_voting_state(pda_state, &voting_state)?;
            try_functions::try_bumps(*bump_voter_provided, bump_voter_, bump)?;
            let mut new_votes = num_votes;
            let mut new_pos1 = pos1;
            let mut new_pos2 = pos2;
            let mut new_neg1 = neg1;

            match vote_preference {
                VotePreference::Negative => {
                    try_functions::try_vote_negative(&mut new_votes, &mut new_neg1, pda_party)?;
                }
                VotePreference::Positive => {
                    try_functions::try_vote_positive(
                        &mut new_votes,
                        &mut new_pos1,
                        &mut new_pos2,
                        pda_party,
                    )?;
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
            try_functions::try_uninitialized(is_initialized)?;
            try_functions::try_voting_state(pda_state, &voting_state)?;
            try_functions::try_bumps(*bump_party_provided, bump_party_, bump)?;
            let mut new_votes = votes;

            match vote_preference {
                VotePreference::Negative => {
                    try_functions::try_decrease_votes(&mut new_votes)?;
                }
                VotePreference::Positive => {
                    try_functions::try_increase_votes(&mut new_votes)?;
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
