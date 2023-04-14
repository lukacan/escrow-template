//#![cfg(feature = "test-bpf")]
use solana_client::client_error::ClientError;
use solana_client::rpc_client::RpcClient;
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_sdk::{
    account::AccountSharedData,
    declare_id,
    signature::{Keypair, Signature},
};

use {
    assert_matches::*,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_sdk::{signature::Signer, transaction::Transaction},
    solana_validator::test_validator::*,
};

declare_id!("Fnambs3f1XXoMmAVc94bf8t6JDAxmVkXz85XU4v2edph");
fn initialize(rpc_client: &RpcClient, initializer: &Keypair) -> Result<Signature, ClientError> {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let (pda_owner, bump_owner) =
        Pubkey::find_program_address(&[b"voting_owner", initializer.pubkey().as_ref()], &id());

    let (pda_state, bump_state) =
        Pubkey::find_program_address(&[b"voting_state", pda_owner.as_ref()], &id());

    let instruction_data = vec![0u8, bump_owner, bump_state];

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(initializer.pubkey(), true), // initializer
                AccountMeta::new(pda_owner, false),           // voting owner
                AccountMeta::new(pda_state, false),           // voting state
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
            data: instruction_data,
        }],
        Some(&initializer.pubkey()),
    );
    transaction.sign(&[initializer], blockhash);

    rpc_client.send_and_confirm_transaction(&transaction)
}

fn create_party(
    rpc_client: &RpcClient,
    initializer: &Keypair,
    person: &Keypair,
    party_name: &String,
) -> Result<Signature, ClientError> {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let (pda_owner, bump_owner) =
        Pubkey::find_program_address(&[b"voting_owner", initializer.pubkey().as_ref()], &id());

    let (pda_state, bump_state) =
        Pubkey::find_program_address(&[b"voting_state", pda_owner.as_ref()], &id());

    let (pda_party, bump_party) =
        Pubkey::find_program_address(&[party_name.as_bytes(), pda_state.as_ref()], &id());

    let mut instruction_data = vec![
        1u8,
        bump_owner,
        bump_state,
        bump_party,
        party_name.chars().count() as u8,
        0u8,
        0u8,
        0u8,
    ];
    for byte in party_name.as_bytes() {
        instruction_data.push(*byte);
    }

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(person.pubkey(), true), // persone that wants to create party
                AccountMeta::new_readonly(initializer.pubkey(), true), // owner
                AccountMeta::new_readonly(pda_owner, false), // voting owner
                AccountMeta::new_readonly(pda_state, false), // voting state
                AccountMeta::new(pda_party, false),      // party
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
            data: instruction_data,
        }],
        Some(&person.pubkey()),
    );
    transaction.sign(&[person, initializer], blockhash);

    rpc_client.send_and_confirm_transaction(&transaction)
}

fn create_voter(
    rpc_client: &RpcClient,
    initializer: &Pubkey,
    voter: &Keypair,
) -> Result<Signature, ClientError> {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let (pda_owner, bump_owner) =
        Pubkey::find_program_address(&[b"voting_owner", initializer.as_ref()], &id());

    let (pda_state, bump_state) =
        Pubkey::find_program_address(&[b"voting_state", pda_owner.as_ref()], &id());

    let (pda_voter, bump_voter) = Pubkey::find_program_address(
        &[b"new_voter", voter.pubkey().as_ref(), pda_state.as_ref()],
        &id(),
    );

    let instruction_data = vec![2u8, bump_owner, bump_state, bump_voter];

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(voter.pubkey(), true), // persone that wants to be voter
                AccountMeta::new_readonly(*initializer, false), // owner
                AccountMeta::new_readonly(pda_owner, false), // voting owner
                AccountMeta::new_readonly(pda_state, false), // voting state
                AccountMeta::new(pda_voter, false),     // voter
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
            data: instruction_data,
        }],
        Some(&voter.pubkey()),
    );
    transaction.sign(&[voter], blockhash);

    rpc_client.send_and_confirm_transaction(&transaction)
}

fn vote_positive(
    rpc_client: &RpcClient,
    initializer: &Pubkey,
    voter: &Keypair,
    party_name: &String,
) -> Result<Signature, ClientError> {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let (pda_owner, bump_owner) =
        Pubkey::find_program_address(&[b"voting_owner", initializer.as_ref()], &id());

    let (pda_state, bump_state) =
        Pubkey::find_program_address(&[b"voting_state", pda_owner.as_ref()], &id());

    let (pda_voter, bump_voter) = Pubkey::find_program_address(
        &[b"new_voter", voter.pubkey().as_ref(), pda_state.as_ref()],
        &id(),
    );
    let (pda_party, bump_party) =
        Pubkey::find_program_address(&[party_name.as_bytes(), pda_state.as_ref()], &id());

    let mut instruction_data = vec![
        3u8,
        bump_owner,
        bump_state,
        bump_voter,
        bump_party,
        party_name.chars().count() as u8,
        0u8,
        0u8,
        0u8,
    ];
    for byte in party_name.as_bytes() {
        instruction_data.push(*byte);
    }

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(voter.pubkey(), true), // person that wants to be voter
                AccountMeta::new_readonly(*initializer, false), // owner
                AccountMeta::new_readonly(pda_owner, false), // voting owner
                AccountMeta::new_readonly(pda_state, false), // voting state
                AccountMeta::new(pda_voter, false),     // voter
                AccountMeta::new(pda_party, false),     // party
            ],
            data: instruction_data,
        }],
        Some(&voter.pubkey()),
    );
    transaction.sign(&[voter], blockhash);

    rpc_client.send_and_confirm_transaction(&transaction)
}

fn vote_negative(
    rpc_client: &RpcClient,
    initializer: &Pubkey,
    voter: &Keypair,
    party_name: &String,
) -> Result<Signature, ClientError> {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let (pda_owner, bump_owner) =
        Pubkey::find_program_address(&[b"voting_owner", initializer.as_ref()], &id());

    let (pda_state, bump_state) =
        Pubkey::find_program_address(&[b"voting_state", pda_owner.as_ref()], &id());

    let (pda_voter, bump_voter) = Pubkey::find_program_address(
        &[b"new_voter", voter.pubkey().as_ref(), pda_state.as_ref()],
        &id(),
    );
    let (pda_party, bump_party) =
        Pubkey::find_program_address(&[party_name.as_bytes(), pda_state.as_ref()], &id());

    let mut instruction_data = vec![
        4u8,
        bump_owner,
        bump_state,
        bump_voter,
        bump_party,
        party_name.chars().count() as u8,
        0u8,
        0u8,
        0u8,
    ];
    for byte in party_name.as_bytes() {
        instruction_data.push(*byte);
    }

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(voter.pubkey(), true), // persone that wants to be voter
                AccountMeta::new_readonly(*initializer, false), // owner
                AccountMeta::new_readonly(pda_owner, false), // voting owner
                AccountMeta::new_readonly(pda_state, false), // voting state
                AccountMeta::new(pda_voter, false),     // voter
                AccountMeta::new(pda_party, false),     // party
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
            data: instruction_data,
        }],
        Some(&voter.pubkey()),
    );
    transaction.sign(&[voter], blockhash);

    rpc_client.send_and_confirm_transaction(&transaction)
}
#[test]
fn test_basic() {
    solana_logger::setup_with_default("solana_program_runtime=debug");

    let mut testvalgen = TestValidatorGenesis::default();

    let initializer = Keypair::new();

    let account = AccountSharedData::new(
        LAMPORTS_PER_SOL * 2,
        0,
        &solana_program::system_program::id(),
    );
    testvalgen.add_account(initializer.pubkey(), account);

    let alice = Keypair::new();

    let account = AccountSharedData::new(
        LAMPORTS_PER_SOL * 2,
        0,
        &solana_program::system_program::id(),
    );
    testvalgen.add_account(alice.pubkey(), account);

    let bob = Keypair::new();

    let account = AccountSharedData::new(
        LAMPORTS_PER_SOL * 2,
        0,
        &solana_program::system_program::id(),
    );
    testvalgen.add_account(bob.pubkey(), account);

    let diana = Keypair::new();

    let account = AccountSharedData::new(
        LAMPORTS_PER_SOL * 2,
        0,
        &solana_program::system_program::id(),
    );
    testvalgen.add_account(diana.pubkey(), account);

    let (test_validator, _payer) = testvalgen
        .add_program("target/deploy/bpf_program_template", id())
        .start();

    solana_logger::setup_with_default("solana_runtime::message=debug");
    let rpc_client = test_validator.get_rpc_client();

    let alice_party: String = String::from("Alice Party");
    let diana_party: String = String::from("Diana Party");

    assert_matches!(initialize(&rpc_client, &initializer), Ok(_));

    assert_matches!(
        create_party(&rpc_client, &initializer, &alice, &alice_party),
        Ok(_)
    );
    assert_matches!(
        create_party(&rpc_client, &initializer, &alice, &diana_party),
        Ok(_)
    );
    assert_matches!(
        create_voter(&rpc_client, &initializer.pubkey(), &bob),
        Ok(_)
    );
    assert_matches!(
        vote_positive(&rpc_client, &initializer.pubkey(), &bob, &alice_party),
        Ok(_)
    );
    assert_matches!(
        vote_positive(&rpc_client, &initializer.pubkey(), &bob, &alice_party),
        Err(_)
    );
    assert_matches!(
        vote_negative(&rpc_client, &initializer.pubkey(), &bob, &alice_party),
        Err(_)
    );
    assert_matches!(
        vote_positive(&rpc_client, &initializer.pubkey(), &bob, &diana_party),
        Ok(_)
    );
    assert_matches!(
        vote_negative(&rpc_client, &initializer.pubkey(), &bob, &alice_party),
        Ok(_)
    );
    assert_matches!(
        vote_negative(&rpc_client, &initializer.pubkey(), &bob, &alice_party),
        Err(_)
    );
}
