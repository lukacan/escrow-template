use borsh::BorshDeserialize;
use bpf_program_template::{
    instruction::{
        create_party, create_voter, get_owner_address, get_party_address, get_state_address,
        get_voter_address, initialize, string_to_bytearray, vote_negative, vote_positive,
    },
    state::{JanecekState, VotesStates},
};
use solana_client::rpc_client::RpcClient;
use solana_program::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
use solana_sdk::{
    account::AccountSharedData, declare_id, signature::Keypair, signature::Signer,
    transaction::Transaction,
};
use solana_validator::test_validator::*;

declare_id!("Fnambs3f1XXoMmAVc94bf8t6JDAxmVkXz85XU4v2edph");

/// initialize TestValidator
#[allow(dead_code)]
pub fn init_env() -> TestValidatorGenesis {
    // solana_logger::setup_with_default("solana_program_runtime=debug");
    // solana_logger::setup_with_default("solana_runtime::message=debug");
    let mut testvalgen = TestValidatorGenesis::default();
    testvalgen.add_program("target/deploy/bpf_program_template", id());
    testvalgen
}

/// Transaction for Initialize Instruction
#[allow(dead_code)]
pub fn initialize_transaction(
    rpc_client: &RpcClient,
    initializer: &Keypair,
) -> Result<solana_sdk::signature::Signature, solana_client::client_error::ClientError> {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[initialize(initializer.pubkey())],
        Some(&initializer.pubkey()),
    );

    transaction.sign(&[initializer], blockhash);
    rpc_client.send_and_confirm_transaction(&transaction)
}

/// Transaction for Create Party Instruction
#[allow(dead_code)]
pub fn create_party_transaction(
    rpc_client: &RpcClient,
    initializer: &Keypair,
    author: &Keypair,
    party_name: &str,
) -> Result<solana_sdk::signature::Signature, solana_client::client_error::ClientError> {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[create_party(
            initializer.pubkey(),
            author.pubkey(),
            String::from(party_name),
        )],
        Some(&author.pubkey()),
    );

    transaction.sign(&[author, initializer], blockhash);
    rpc_client.send_and_confirm_transaction(&transaction)
}

/// Transaction for Create Voter Instruction
#[allow(dead_code)]
pub fn create_voter_transaction(
    rpc_client: &RpcClient,
    initializer: &Keypair,
    author: &Keypair,
) -> Result<solana_sdk::signature::Signature, solana_client::client_error::ClientError> {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[create_voter(initializer.pubkey(), author.pubkey())],
        Some(&author.pubkey()),
    );

    transaction.sign(&[author], blockhash);
    rpc_client.send_and_confirm_transaction(&transaction)
}

/// Transaction for Vote Positive Instruction
#[allow(dead_code)]
pub fn create_vote_pos_transaction(
    rpc_client: &RpcClient,
    initializer: &Keypair,
    author: &Keypair,
    party_name: &str,
) -> Result<solana_sdk::signature::Signature, solana_client::client_error::ClientError> {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[vote_positive(
            initializer.pubkey(),
            author.pubkey(),
            String::from(party_name),
        )],
        Some(&author.pubkey()),
    );

    transaction.sign(&[author], blockhash);
    rpc_client.send_and_confirm_transaction(&transaction)
}

/// Transaction for Vote Negative Instruction
#[allow(dead_code)]
pub fn create_vote_neg_transaction(
    rpc_client: &RpcClient,
    initializer: &Keypair,
    author: &Keypair,
    party_name: &str,
) -> Result<solana_sdk::signature::Signature, solana_client::client_error::ClientError> {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[vote_negative(
            initializer.pubkey(),
            author.pubkey(),
            String::from(party_name),
        )],
        Some(&author.pubkey()),
    );

    transaction.sign(&[author], blockhash);
    rpc_client.send_and_confirm_transaction(&transaction)
}

/// Add account into Test Validator
#[allow(dead_code)]
pub fn add_account(testvalgen: &mut TestValidatorGenesis) -> Keypair {
    let alice = Keypair::new();
    let account = AccountSharedData::new(
        LAMPORTS_PER_SOL * 2,
        0,
        &solana_program::system_program::id(),
    );
    testvalgen.add_account(alice.pubkey(), account);
    alice
}

/// Deserialize Account Data
#[allow(dead_code)]
pub fn de_account_data(
    account_data: &mut &[u8],
) -> Option<bpf_program_template::state::JanecekState> {
    match JanecekState::deserialize(account_data).unwrap() {
        JanecekState::VotingOwner {
            is_initialized,
            author,
            voting_state,
            bump,
        } => Some(JanecekState::VotingOwner {
            is_initialized,
            author,
            voting_state,
            bump,
        }),
        JanecekState::Fresh => None,
        JanecekState::Party {
            is_initialized,
            author,
            voting_state,
            created,
            name,
            votes,
            bump,
        } => Some(JanecekState::Party {
            is_initialized,
            author,
            voting_state,
            created,
            name,
            votes,
            bump,
        }),
        JanecekState::Voter {
            is_initialized,
            author,
            voting_state,
            num_votes,
            pos1,
            pos2,
            neg1,
            bump,
        } => Some(JanecekState::Voter {
            is_initialized,
            author,
            voting_state,
            num_votes,
            pos1,
            pos2,
            neg1,
            bump,
        }),
        JanecekState::VotingState {
            is_initialized,
            voting_owner,
            voting_started,
            voting_ends,
            bump,
        } => Some(JanecekState::VotingState {
            is_initialized,
            voting_owner,
            voting_started,
            voting_ends,
            bump,
        }),
    }
}

/// Check if Voting Owner data corresponds to what we expect
#[allow(dead_code)]
pub fn compare_voting_owner_data(rpc_client: &RpcClient, initializer: &Keypair) {
    let (pda_owner, _owner_bump) = get_owner_address(initializer.pubkey());
    let (pda_state, _state_bump) = get_state_address(pda_owner);

    let voting_owner_acc = rpc_client.get_account(&pda_owner).unwrap();
    let voting_owner_data = de_account_data(&mut voting_owner_acc.data.as_slice()).unwrap();

    assert_eq!(voting_owner_acc.owner, id());
    assert_eq!(
        voting_owner_data,
        JanecekState::VotingOwner {
            is_initialized: true,
            author: initializer.pubkey(),
            voting_state: pda_state,
            bump: _owner_bump
        }
    );
}
/// Check if Voting Owner data corresponds to what we expect

/// Check if Voting State data corresponds to what we expect
#[allow(dead_code)]
pub fn compare_voting_state_data(rpc_client: &RpcClient, initializer: &Keypair) {
    let (pda_owner, _owner_bump) = get_owner_address(initializer.pubkey());
    let (pda_state, _state_bump) = get_state_address(pda_owner);

    let voting_state_acc = rpc_client.get_account(&pda_state).unwrap();
    let voting_state_data = de_account_data(&mut voting_state_acc.data.as_slice()).unwrap();

    assert_eq!(voting_state_acc.owner, id());
    match voting_state_data {
        bpf_program_template::state::JanecekState::VotingState {
            is_initialized,
            voting_owner,
            voting_started,
            voting_ends,
            bump,
        } => {
            assert!(is_initialized);
            assert_eq!(voting_owner, pda_owner);
            assert_eq!(voting_ends - voting_started, JanecekState::VOTING_LENGTH);
            assert_eq!(bump, _state_bump);
        }
        _ => {
            assert_eq!(false, true);
        }
    }
}

/// Check if Party data corresponds to what we expect
#[allow(dead_code)]
pub fn compare_party_data(
    rpc_client: &RpcClient,
    initializer: &Keypair,
    party_author: &Keypair,
    party_name: &str,
    num_votes: i64,
) {
    let (pda_owner, _owner_bump) = get_owner_address(initializer.pubkey());
    let (pda_state, _state_bump) = get_state_address(pda_owner);
    let name_bytearray = string_to_bytearray(String::from(party_name));
    let (pda_party, _party_bump) = get_party_address(&name_bytearray, pda_state);

    let party_acc = rpc_client.get_account(&pda_party).unwrap();

    assert_eq!(party_acc.owner, id());

    let party_data = de_account_data(&mut party_acc.data.as_slice()).unwrap();

    match party_data {
        bpf_program_template::state::JanecekState::Party {
            is_initialized,
            author,
            voting_state,
            created: _,
            name,
            votes,
            bump,
        } => {
            assert!(is_initialized);
            assert_eq!(author, party_author.pubkey());
            assert_eq!(voting_state, pda_state);
            assert_eq!(name, name_bytearray);
            assert_eq!(votes, num_votes);
            assert_eq!(bump, _party_bump);
        }
        _ => {
            assert_eq!(false, true);
        }
    }
}

/// Check if Voter data corresponds to what we expect
#[allow(dead_code)]
pub fn compare_voter_data(
    rpc_client: &RpcClient,
    initializer: &Keypair,
    bob: &Keypair,
    num_votes_ref: VotesStates,
    pos1_ref: Pubkey,
    pos2_ref: Pubkey,
    neg1_ref: Pubkey,
) {
    let (pda_owner, _owner_bump) = get_owner_address(initializer.pubkey());
    let (pda_state, _state_bump) = get_state_address(pda_owner);
    let (pda_voter, _voter_bump) = get_voter_address(bob.pubkey(), pda_state);

    let voter_acc = rpc_client.get_account(&pda_voter).unwrap();
    assert_eq!(voter_acc.owner, id());
    let voter_data = de_account_data(&mut voter_acc.data.as_slice()).unwrap();
    assert_eq!(
        voter_data,
        JanecekState::Voter {
            is_initialized: true,
            author: bob.pubkey(),
            voting_state: pda_state,
            num_votes: num_votes_ref,
            pos1: pos1_ref,
            pos2: pos2_ref,
            neg1: neg1_ref,
            bump: _voter_bump
        }
    );
}
