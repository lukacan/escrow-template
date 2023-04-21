use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use crate::entrypoint::id;
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum VoteContext {
    Positive,
    Negative,
}
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum JanecekInstruction {
    Initialize,
    CreateParty {
        bump_owner: u8,
        bump_state: u8,
        name: String,
    },
    CreateVoter {
        bump_owner: u8,
        bump_state: u8,
    },
    Vote {
        bump_owner: u8,
        bump_state: u8,
        bump_voter: u8,
        bump_party: u8,
        vote_context: VoteContext,
        name: String,
    },
}

pub fn get_owner_address(account: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"voting_owner", account.as_ref()], &id())
}
pub fn get_party_address(name: &String, account: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[name.as_bytes(), account.as_ref()], &id())
}
pub fn get_voter_address(author: Pubkey, account: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"new_voter", author.as_ref(), account.as_ref()], &id())
}
pub fn get_state_address(account: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"voting_state", account.as_ref()], &id())
}

/// API call that generates instruction for Initialize
pub fn initialize(initializer: Pubkey) -> Instruction {
    let (owner, _bump_owner) = get_owner_address(initializer);
    let (state, _bump_state) = get_state_address(owner);
    Instruction {
        program_id: id(),
        accounts: vec![
            AccountMeta::new(initializer, true),
            AccountMeta::new(owner, false),
            AccountMeta::new(state, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: JanecekInstruction::Initialize.try_to_vec().unwrap(),
    }
}
/// API call that generates instruction for Create Party
pub fn create_party(initializer: Pubkey, party_author: Pubkey, name: String) -> Instruction {
    let (owner, bump_owner) = get_owner_address(initializer);
    let (state, bump_state) = get_state_address(owner);
    let (party, _bump_party) = get_party_address(&name, state);

    Instruction {
        program_id: id(),
        accounts: vec![
            AccountMeta::new(party_author, true),
            AccountMeta::new_readonly(initializer, true),
            AccountMeta::new_readonly(owner, false),
            AccountMeta::new_readonly(state, false),
            AccountMeta::new(party, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: JanecekInstruction::CreateParty {
            bump_owner,
            bump_state,
            name,
        }
        .try_to_vec()
        .unwrap(),
    }
}
/// API call that generates instruction for Create Voter
pub fn create_voter(initializer: Pubkey, voter_author: Pubkey) -> Instruction {
    let (owner, bump_owner) = get_owner_address(initializer);
    let (state, bump_state) = get_state_address(owner);
    let (voter, _bump_voter) = get_voter_address(voter_author, state);

    Instruction {
        program_id: id(),
        accounts: vec![
            AccountMeta::new(voter_author, true),
            AccountMeta::new_readonly(initializer, false),
            AccountMeta::new_readonly(owner, false),
            AccountMeta::new_readonly(state, false),
            AccountMeta::new(voter, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: JanecekInstruction::CreateVoter {
            bump_owner,
            bump_state,
        }
        .try_to_vec()
        .unwrap(),
    }
}
/// API call that generates instruction for Vote Positive
pub fn vote_positive(initializer: Pubkey, voter_author: Pubkey, name: String) -> Instruction {
    let (owner, bump_owner) = get_owner_address(initializer);
    let (state, bump_state) = get_state_address(owner);
    let (voter, bump_voter) = get_voter_address(voter_author, state);
    let (party, bump_party) = get_party_address(&name, state);

    Instruction {
        program_id: id(),
        accounts: vec![
            AccountMeta::new(voter_author, true),
            AccountMeta::new_readonly(initializer, false),
            AccountMeta::new_readonly(owner, false),
            AccountMeta::new_readonly(state, false),
            AccountMeta::new(voter, false),
            AccountMeta::new(party, false),
        ],
        data: JanecekInstruction::Vote {
            bump_owner,
            bump_state,
            bump_voter,
            bump_party,
            vote_context: VoteContext::Positive,
            name,
        }
        .try_to_vec()
        .unwrap(),
    }
}
/// API call that generates instruction for Vote Negative
pub fn vote_negative(initializer: Pubkey, voter_author: Pubkey, name: String) -> Instruction {
    let (owner, bump_owner) = get_owner_address(initializer);
    let (state, bump_state) = get_state_address(owner);
    let (voter, bump_voter) = get_voter_address(voter_author, state);
    let (party, bump_party) = get_party_address(&name, state);

    Instruction {
        program_id: id(),
        accounts: vec![
            AccountMeta::new(voter_author, true),
            AccountMeta::new_readonly(initializer, false),
            AccountMeta::new_readonly(owner, false),
            AccountMeta::new_readonly(state, false),
            AccountMeta::new(voter, false),
            AccountMeta::new(party, false),
        ],
        data: JanecekInstruction::Vote {
            bump_owner,
            bump_state,
            bump_voter,
            bump_party,
            vote_context: VoteContext::Negative,
            name,
        }
        .try_to_vec()
        .unwrap(),
    }
}
