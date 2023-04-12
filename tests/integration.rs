//#![cfg(feature = "test-bpf")]

use solana_client::rpc_client::RpcClient;
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_sdk::{account::AccountSharedData, declare_id, signature::Keypair};

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
fn initialize(rpc_client: &RpcClient, initializer: &Keypair) {
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
                AccountMeta::new(initializer.pubkey(), true),   // initializer
                AccountMeta::new(pda_owner, false),             // voting owner
                AccountMeta::new(pda_state, false),             // voting state
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
            data: instruction_data,
        }],
        Some(&initializer.pubkey()),
    );
    transaction.sign(&[initializer], blockhash);

    assert_matches!(rpc_client.send_and_confirm_transaction(&transaction), Ok(_));
}

fn create_party(rpc_client: &RpcClient, initializer: &Keypair, person: &Keypair) {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let (pda_owner, bump_owner) =
        Pubkey::find_program_address(&[b"voting_owner", initializer.pubkey().as_ref()], &id());

    let (pda_state, bump_state) =
        Pubkey::find_program_address(&[b"voting_state", pda_owner.as_ref()], &id());

    let name: &str = "Party1";

    let (pda_party, bump_party) = Pubkey::find_program_address(
        &[name.as_bytes(),pda_state.as_ref()], 
        &id());

    let mut instruction_data = vec![
        1u8,
        bump_owner,
        bump_state,
        bump_party, 
        name.chars().count() as u8, 
        0u8, 
        0u8, 
        0u8];
    for byte in name.as_bytes() {
        instruction_data.push(*byte);
    }

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(person.pubkey(), true),            // persone that wants to create party
                AccountMeta::new_readonly(initializer.pubkey(), true),       // owner
                AccountMeta::new_readonly(pda_owner, false),        // voting owner
                AccountMeta::new_readonly(pda_state, false),        // voting state
                AccountMeta::new(pda_party, false),                 // party
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
            data: instruction_data,
        }],
        Some(&person.pubkey()),
    );
    transaction.sign(&[person, initializer], blockhash);

    assert_matches!(rpc_client.send_and_confirm_transaction(&transaction), Ok(_));
}

fn create_voter(rpc_client: &RpcClient, payer: &Keypair) {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let (pda, bump) = Pubkey::find_program_address(&[b"new_voter", payer.pubkey().as_ref()], &id());

    let instruction_data = vec![2u8, bump];

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(pda, false),
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
            data: instruction_data,
        }],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer], blockhash);

    assert_matches!(rpc_client.send_and_confirm_transaction(&transaction), Ok(_));
}
#[test]
fn test_validator_transaction() {
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

    let (test_validator, _payer) = testvalgen
        .add_program("target/deploy/bpf_program_template", id())
        .start();

    solana_logger::setup_with_default("solana_runtime::message=debug");
    let rpc_client = test_validator.get_rpc_client();

    initialize(&rpc_client, &initializer);
    create_party(&rpc_client, &initializer ,&alice);
    // create_voter(&rpc_client, &payer);
}
