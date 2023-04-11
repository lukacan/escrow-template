//#![cfg(feature = "test-bpf")]

use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

use {
    assert_matches::*,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_sdk::{signature::Signer, transaction::Transaction},
    solana_validator::test_validator::*,
};

pub const PROGRAM_ID: Pubkey =
    solana_program::pubkey!("Fnambs3f1XXoMmAVc94bf8t6JDAxmVkXz85XU4v2edph");

fn create_party(rpc_client: &RpcClient, payer: &Keypair) {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let name: &str = "Hello";

    let (pda, bump) = Pubkey::find_program_address(&[name.as_bytes()], &PROGRAM_ID);

    let mut instruction_data = vec![0u8, name.chars().count() as u8, 0u8, 0u8, 0u8];
    for byte in name.as_bytes() {
        instruction_data.push(*byte);
    }
    instruction_data.push(bump);

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: PROGRAM_ID,
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

fn create_voter(rpc_client: &RpcClient, payer: &Keypair) {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let (pda, bump) =
        Pubkey::find_program_address(&[b"new_voter", payer.pubkey().as_ref()], &PROGRAM_ID);

    let instruction_data = vec![1u8, bump];

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: PROGRAM_ID,
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

    let (test_validator, payer) = TestValidatorGenesis::default()
        .add_program("target/deploy/bpf_program_template", PROGRAM_ID)
        .start();

    solana_logger::setup_with_default("solana_runtime::message=debug");
    let rpc_client = test_validator.get_rpc_client();

    let balance = rpc_client.get_account(&payer.pubkey()).unwrap().lamports;

    println!("Sol balance of payer is {balance}");

    create_party(&rpc_client, &payer);
    create_voter(&rpc_client, &payer);
}
