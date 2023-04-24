use borsh::BorshDeserialize;
use bpf_program_template::{
    instruction::{create_party, create_voter, initialize, vote_negative, vote_positive},
    state::JanecekState,
};
use solana_client::rpc_client::RpcClient;
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_sdk::{
    account::AccountSharedData, declare_id, signature::Keypair, signature::Signer,
    transaction::Transaction,
};
use solana_validator::test_validator::*;

declare_id!("Fnambs3f1XXoMmAVc94bf8t6JDAxmVkXz85XU4v2edph");

#[allow(dead_code)]
pub fn init_env() -> TestValidatorGenesis {
    // solana_logger::setup_with_default("solana_program_runtime=debug");
    // solana_logger::setup_with_default("solana_runtime::message=debug");
    let mut testvalgen = TestValidatorGenesis::default();
    testvalgen.add_program("target/deploy/bpf_program_template", id());
    testvalgen
}
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
