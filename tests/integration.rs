use bpf_program_template::instruction::{
    create_party, create_voter, get_owner_address, initialize, vote_positive,
};
use solana_client::rpc_client::RpcClient;
// #![cfg(feature = "test-sbf")]
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_sdk::{account::AccountSharedData, declare_id, signature::Keypair};

use {
    solana_sdk::{signature::Signer, transaction::Transaction},
    solana_validator::test_validator::*,
};

use assert_matches::*;

declare_id!("Fnambs3f1XXoMmAVc94bf8t6JDAxmVkXz85XU4v2edph");

fn init_env() -> TestValidatorGenesis {
    // solana_logger::setup_with_default("solana_program_runtime=debug");
    // solana_logger::setup_with_default("solana_runtime::message=debug");
    let mut testvalgen = TestValidatorGenesis::default();
    testvalgen.add_program("target/deploy/bpf_program_template", id());
    testvalgen
}

fn initialize_transaction(
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

fn create_party_transaction(
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

fn create_voter_transaction(
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

fn create_vote_pos_transaction(
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

fn add_account(testvalgen: &mut TestValidatorGenesis) -> Keypair {
    let alice = Keypair::new();
    let account = AccountSharedData::new(
        LAMPORTS_PER_SOL * 2,
        0,
        &solana_program::system_program::id(),
    );
    testvalgen.add_account(alice.pubkey(), account);
    alice
}

#[test]
fn test_initialize_basic1() {
    let mut testvalgen = init_env();
    let initializer = add_account(&mut testvalgen);

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(initialize_transaction(&rpc_client, &initializer), Ok(_));
}

#[test]
fn test_initialize_basic2() {
    let mut testvalgen = init_env();
    let initializer = add_account(&mut testvalgen);

    let pda_owner = get_owner_address(initializer.pubkey());

    let pda_owner_acc = AccountSharedData::new(
        LAMPORTS_PER_SOL * 2,
        0,
        &solana_program::system_program::id(),
    );
    testvalgen.add_account(pda_owner.0, pda_owner_acc);

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(initialize_transaction(&rpc_client, &initializer), Ok(_));
}

#[test]
fn test_initialize_basic3() {
    let mut testvalgen = init_env();
    let initializer = add_account(&mut testvalgen);

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(initialize_transaction(&rpc_client, &initializer), Ok(_));
    assert_matches!(initialize_transaction(&rpc_client, &initializer), Err(_));
}

#[test]
fn test_create_party_basic1() {
    let mut testvalgen = init_env();
    let initializer = add_account(&mut testvalgen);
    let alice = add_account(&mut testvalgen);
    let party_name = "Alice Party";

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(initialize_transaction(&rpc_client, &initializer), Ok(_));
    assert_matches!(
        create_party_transaction(&rpc_client, &initializer, &alice, party_name,),
        Ok(_)
    );
}

#[test]
fn test_create_party_basic2() {
    let mut testvalgen = init_env();
    let initializer = add_account(&mut testvalgen);
    let alice = add_account(&mut testvalgen);
    let party_name = "Alice Party";

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(initialize_transaction(&rpc_client, &initializer), Ok(_));
    assert_matches!(
        create_party_transaction(&rpc_client, &initializer, &alice, party_name,),
        Ok(_)
    );

    assert_matches!(
        create_party_transaction(&rpc_client, &initializer, &alice, party_name,),
        Err(_)
    );
}

#[test]
fn test_create_voter_basic1() {
    let mut testvalgen = init_env();
    let initializer = add_account(&mut testvalgen);
    let bob = add_account(&mut testvalgen);

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(initialize_transaction(&rpc_client, &initializer), Ok(_));
    assert_matches!(
        create_voter_transaction(&rpc_client, &initializer, &bob),
        Ok(_)
    );
}
#[test]
fn test_try_pos_vote_basic1() {
    let mut testvalgen = init_env();
    let initializer = add_account(&mut testvalgen);
    let bob = add_account(&mut testvalgen);
    let alice = add_account(&mut testvalgen);
    let party_name = "Alice Party";

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(initialize_transaction(&rpc_client, &initializer), Ok(_));
    assert_matches!(
        create_voter_transaction(&rpc_client, &initializer, &bob),
        Ok(_)
    );
    assert_matches!(
        create_party_transaction(&rpc_client, &initializer, &alice, party_name),
        Ok(_)
    );
    assert_matches!(
        create_vote_pos_transaction(&rpc_client, &initializer, &bob, party_name),
        Ok(_)
    );
}

#[test]
fn test_try_pos_vote_basic2() {
    let mut testvalgen = init_env();
    let initializer = add_account(&mut testvalgen);
    let bob = add_account(&mut testvalgen);
    let alice = add_account(&mut testvalgen);
    let ben = add_account(&mut testvalgen);
    let party_alice = "Alice Party";
    let party_ben = "Ben Party";

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(initialize_transaction(&rpc_client, &initializer), Ok(_));
    assert_matches!(
        create_voter_transaction(&rpc_client, &initializer, &bob),
        Ok(_)
    );
    assert_matches!(
        create_party_transaction(&rpc_client, &initializer, &alice, party_alice),
        Ok(_)
    );
    assert_matches!(
        create_vote_pos_transaction(&rpc_client, &initializer, &bob, party_alice),
        Ok(_)
    );
    assert_matches!(
        create_party_transaction(&rpc_client, &initializer, &ben, party_ben),
        Ok(_)
    );
    assert_matches!(
        create_vote_pos_transaction(&rpc_client, &initializer, &bob, party_ben),
        Ok(_)
    );
}
