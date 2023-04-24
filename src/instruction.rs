use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use std::mem::size_of;

use crate::{entrypoint::id, state::JanecekState};
#[derive(Debug, BorshDeserialize, BorshSerialize)]
#[repr(u8)]
pub enum VotePreference {
    Positive,
    Negative,
}
#[derive(Debug, BorshDeserialize, BorshSerialize)]
#[repr(u8)]
pub enum JanecekInstruction {
    /// Starts voting by initializing Voting State that is tied to Voting Owner, Voting State is automatically initialized
    /// with 7 days deadline, that means, during this period of time, new Parties, new Voters can be added to this Voting
    /// context same as Voting can be performed. After the deadline, no more Parties or Voters can be added to the current
    /// Voting Context, same as no voting can be performed
    ///
    ///
    /// Accounts expected:
    ///
    /// 0️⃣ `[signer]` The account of the person initializing the voting
    ///
    /// 1️⃣ `[writable]` Program Derived Address which serves as Voting Owner and is tied up to Voting State
    ///
    /// 2️⃣ `[writable]` Program Derived Address which serves as Voting State and is tied up to Voting Owner
    ///
    /// 4️⃣ `[]` The system program
    Initialize,
    /// Creates Party in the specified Voting Context (Voting Owner and Voting State), party can be created only with
    /// the consent of the owner (account that Initialized this Context). Party name has to be uniqe in this Voting Context
    /// It is not allowed to create new Party after voting ended in the Context.
    ///
    ///
    /// Accounts expected:
    ///
    /// 0️⃣ `[signer]` The account that wants to create new Party
    ///
    /// 1️⃣ `[signer]` Owner of the specified Voting Context
    ///
    /// 2️⃣ `[]` Program Derived Address which serves as Voting Owner and is tied up to Voting State
    ///
    /// 3️⃣ `[]` Program Derived Address which serves as Voting State and is tied up to Voting Owner
    ///
    /// 4️⃣ `[writable]` Program Derived Address which serves as Party data account that stores information about votes etc.
    ///
    /// 5️⃣ `[]` The system program
    CreateParty {
        bump_owner: u8,
        bump_state: u8,
        name_bytearray: [u8; JanecekState::NAME_LENGTH],
    },
    /// Creates Voter in the specified Voting Context, meaning if person wants to vote, he has to call this function first,
    /// to initialize his voting data account that stores info about how many free votes he had spent, for which parties
    /// he voted etc.
    ///
    ///
    /// Accounts expected:
    ///
    /// 0️⃣ `[signer]` The account that wants to Vote
    ///
    /// 1️⃣ `[]` Owner of the specified Voting Context
    ///
    /// 2️⃣ `[]` Program Derived Address which serves as Voting Owner and is tied up to Voting State
    ///
    /// 3️⃣ `[]` Program Derived Address which serves as Voting State and is tied up to Voting Owner
    ///
    /// 4️⃣ `[writable]` Program Derived Address which serves as Voter data account that stores information about free votes etc.
    ///
    /// 5️⃣ `[]` The system program
    CreateVoter { bump_owner: u8, bump_state: u8 },
    /// Vote Positive for given Party in given Voting Context
    ///
    ///
    /// Accounts expected:
    ///
    /// 0️⃣ `[signer]` The account that wants to Vote
    ///
    /// 1️⃣ `[]` Owner of the specified Voting Context
    ///
    /// 2️⃣ `[]` Program Derived Address which serves as Voting Owner and is tied up to Voting State
    ///
    /// 3️⃣ `[]` Program Derived Address which serves as Voting State and is tied up to Voting Owner
    ///
    /// 4️⃣ `[writable]` Program Derived Address which serves as Voter data account that stores information about free votes etc.
    ///
    /// 5️⃣ `[writable]` Program Derived Address which serves as Party data account that stores information about votes etc.
    VotePos {
        bump_owner: u8,
        bump_state: u8,
        bump_voter: u8,
        bump_party: u8,
        name_bytearray: [u8; JanecekState::NAME_LENGTH],
    },
    /// Vote Negative for given Party in given Voting Context
    ///
    ///
    /// Accounts expected:
    ///
    /// 0️⃣ `[signer]` The account that wants to Vote
    ///
    /// 1️⃣ `[]` Owner of the specified Voting Context
    ///
    /// 2️⃣ `[]` Program Derived Address which serves as Voting Owner and is tied up to Voting State
    ///
    /// 3️⃣ `[]` Program Derived Address which serves as Voting State and is tied up to Voting Owner
    ///
    /// 4️⃣ `[writable]` Program Derived Address which serves as Voter data account that stores information about free votes etc.
    ///
    /// 5️⃣ `[writable]` Program Derived Address which serves as Party data account that stores information about votes etc.
    VoteNeg {
        bump_owner: u8,
        bump_state: u8,
        bump_voter: u8,
        bump_party: u8,
        name_bytearray: [u8; JanecekState::NAME_LENGTH],
    },
}

impl JanecekInstruction {
    /// specifies expected length of initialize instruction data
    pub const INIT_LEN: usize = size_of::<u8>(); // tag

    /// specifies expected length of create party instruction data
    pub const C_PARTY_LEN: usize = size_of::<u8>() // tag
        + size_of::<u8>() // bump owner
        + size_of::<u8>() // bump state
        + size_of::<u8>() * JanecekState::NAME_LENGTH; // party name

    /// specifies expected length of create voter instruction data
    pub const C_VOTER_LEN: usize = size_of::<u8>() // tag
        + size_of::<u8>() // bump owner
        + size_of::<u8>(); // bump state

    /// specifies expected length of vote positive instruction data
    pub const VOTE_P_LEN: usize = size_of::<u8>() // tag
        + size_of::<u8>() // bump owner
        + size_of::<u8>() // bump state
        + size_of::<u8>() // bump voter
        + size_of::<u8>() // bump party
        + size_of::<u8>() * JanecekState::NAME_LENGTH; // party name

    /// specifies expected length of vote negative instruction data
    pub const VOTE_N_LEN: usize = size_of::<u8>() // tag
        + size_of::<u8>() // bump owner
        + size_of::<u8>() // bump state
        + size_of::<u8>() // bump voter
        + size_of::<u8>() // bump party
        + size_of::<u8>() * JanecekState::NAME_LENGTH; // party name
}

pub fn string_to_bytearray(name: String) -> [u8; JanecekState::NAME_LENGTH] {
    let mut name_bytearray: [u8; JanecekState::NAME_LENGTH] = [0u8; JanecekState::NAME_LENGTH];
    name_bytearray[..name.len()].copy_from_slice(name.into_bytes().as_slice());
    name_bytearray
}
pub fn get_owner_address(account: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"voting_owner", account.as_ref()], &id())
}
pub fn get_party_address(
    name_bytearray: &[u8; JanecekState::NAME_LENGTH],
    account: Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[name_bytearray, account.as_ref()], &id())
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
    let name_bytearray = string_to_bytearray(name);

    let (owner, bump_owner) = get_owner_address(initializer);
    let (state, bump_state) = get_state_address(owner);
    let (party, _bump_party) = get_party_address(&name_bytearray, state);

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
            name_bytearray,
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
/// wrapper around vote instruction
pub fn vote(
    initializer: Pubkey,
    voter_author: Pubkey,
    name: String,
    preference: VotePreference,
) -> Instruction {
    let name_bytearray = string_to_bytearray(name);
    let (owner, bump_owner) = get_owner_address(initializer);
    let (state, bump_state) = get_state_address(owner);
    let (voter, bump_voter) = get_voter_address(voter_author, state);
    let (party, bump_party) = get_party_address(&name_bytearray, state);

    let data = match preference {
        VotePreference::Negative => JanecekInstruction::VoteNeg {
            bump_owner,
            bump_state,
            bump_voter,
            bump_party,
            name_bytearray,
        },
        VotePreference::Positive => JanecekInstruction::VotePos {
            bump_owner,
            bump_state,
            bump_voter,
            bump_party,
            name_bytearray,
        },
    };

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
        // maybe take preference from instruction data
        data: data.try_to_vec().unwrap(),
    }
}
/// API call that generates instruction for Vote Positive
pub fn vote_positive(initializer: Pubkey, voter_author: Pubkey, name: String) -> Instruction {
    vote(initializer, voter_author, name, VotePreference::Positive)
}
/// API call that generates instruction for Vote Negative
pub fn vote_negative(initializer: Pubkey, voter_author: Pubkey, name: String) -> Instruction {
    vote(initializer, voter_author, name, VotePreference::Negative)
}
