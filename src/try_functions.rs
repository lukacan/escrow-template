use crate::entrypoint::id;
use crate::error::JanecekError;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar,
};

use crate::state::VotesStates;

/// check if account is Signer
pub fn try_signer(account: &AccountInfo) -> ProgramResult {
    if !account.is_signer {
        Err(ProgramError::MissingRequiredSignature)
    } else {
        Ok(())
    }
}
/// check if account owner is Program ID
pub fn try_owner(account: &AccountInfo) -> ProgramResult {
    if *account.owner != id() {
        Err(ProgramError::IllegalOwner)
    } else {
        Ok(())
    }
}

/// check if account is rent exempt
pub fn try_rent_exempt(account: &AccountInfo) -> ProgramResult {
    let rent = Rent::get()?;
    if !rent.is_exempt(account.lamports(), account.try_data_len()?) {
        Err(ProgramError::AccountNotRentExempt)
    } else {
        Ok(())
    }
}

/// check if provided PDA equals to derived PDA
pub fn try_seeds(provided: &AccountInfo, derived: &Pubkey) -> ProgramResult {
    if provided.key != derived {
        Err(ProgramError::InvalidSeeds)
    } else {
        Ok(())
    }
}

/// check that provided, derived and saved bumps are equal
pub fn try_bumps(provided: u8, derived: u8, saved: u8) -> ProgramResult {
    if provided != derived || saved != derived {
        Err(ProgramError::InvalidSeeds)
    } else {
        Ok(())
    }
}
/// check if account Pubkey is equal to native System Program ID
pub fn try_system_program(account: &AccountInfo) -> ProgramResult {
    if *account.key != solana_program::system_program::id() {
        Err(ProgramError::IncorrectProgramId)
    } else {
        Ok(())
    }
}

/// check if account is already initialized
pub fn try_initialized(is_initialized: bool) -> ProgramResult {
    if is_initialized {
        Err(ProgramError::AccountAlreadyInitialized)
    } else {
        Ok(())
    }
}

/// check if account is not initialized yet
pub fn try_uninitialized(is_initialized: bool) -> ProgramResult {
    if !is_initialized {
        Err(ProgramError::UninitializedAccount)
    } else {
        Ok(())
    }
}
/// check if instruction data length is correct
pub fn try_ixdata_len(ix_data: &[u8], instr_len: usize) -> ProgramResult {
    if ix_data.len() != instr_len {
        Err(ProgramError::InvalidInstructionData)
    } else {
        Ok(())
    }
}
/// check if voting ended for given timestamp
pub fn try_voting_ended(deadline: i64, timestamp: i64) -> ProgramResult {
    if timestamp > deadline {
        Err(JanecekError::VotingEnded.into())
    } else {
        Ok(())
    }
}
/// check if data accounts`s author and author account provided are same
pub fn try_author(author: &AccountInfo, data_author: &Pubkey) -> ProgramResult {
    if *author.key != *data_author {
        Err(JanecekError::AuthorMismatch.into())
    } else {
        Ok(())
    }
}
/// check if provided PDA voting state address corresponds to Pubkey stored in data account
pub fn try_voting_state(pda_state: &AccountInfo, voting_state: &Pubkey) -> ProgramResult {
    if *pda_state.key != *voting_state {
        Err(JanecekError::VotingStateMismatch.into())
    } else {
        Ok(())
    }
}
/// check if provided PDA voting owner address corresponds to Pubkey stored in data account
pub fn try_voting_owner(pda_owner: &AccountInfo, voting_owner: &Pubkey) -> ProgramResult {
    if *pda_owner.key != *voting_owner {
        Err(JanecekError::VotingOwnerMismatch.into())
    } else {
        Ok(())
    }
}
/// function provides vote positive checks
pub fn try_vote_positive(
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
            if pos1 == pda_party.key {
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

/// function provides vote negative checks
pub fn try_vote_negative(
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

/// function provides checked add
pub fn try_checked_add(base: &mut i64, to_add: i64) -> ProgramResult {
    match base.checked_add(to_add) {
        Some(sucess) => {
            *base = sucess;
            Ok(())
        }
        None => Err(JanecekError::AdditionOverflow.into()),
    }
}
/// function provides checked sub
pub fn try_checked_sub(base: &mut i64, to_sub: i64) -> ProgramResult {
    match base.checked_sub(to_sub) {
        Some(sucess) => {
            *base = sucess;
            Ok(())
        }
        None => Err(JanecekError::SubtractionOverflow.into()),
    }
}
/// create_account or transfer/allocate/assign
pub fn try_create_or_assign(
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
