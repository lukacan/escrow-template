//#![cfg(feature = "test-bpf")]

use std::str::FromStr;

use {
    assert_matches::*,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_sdk::{signature::Signer, transaction::Transaction},
    solana_validator::test_validator::*,
};

#[test]
fn test_validator_transaction() {
    solana_logger::setup_with_default("solana_program_runtime=debug");

    let program_id = Pubkey::from_str("Fnambs3f1XXoMmAVc94bf8t6JDAxmVkXz85XU4v2edph").unwrap();

    let (test_validator, payer) = TestValidatorGenesis::default()
        .add_program("target/deploy/bpf_program_template", program_id)
        .start();

    solana_logger::setup_with_default("solana_runtime::message=debug");
    let rpc_client = test_validator.get_rpc_client();

    let balance = rpc_client.get_account(&payer.pubkey()).unwrap().lamports;

    println!("Sol balance of payer is {balance}");

    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let name: &str = "Hello";

    let (pda, bump) = Pubkey::find_program_address(&[name.as_bytes()], &program_id);

    let mut instruction_data = Vec::<u8>::new();
    instruction_data.push(0u8);
    instruction_data.push(5u8);
    instruction_data.push(0u8);
    instruction_data.push(0u8);
    instruction_data.push(0u8);

    for byte in name.as_bytes() {
        instruction_data.push(*byte);
    }

    instruction_data.push(bump);

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(pda, false),
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
            data: instruction_data,
        }],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], blockhash);

    assert_matches!(rpc_client.send_and_confirm_transaction(&transaction), Ok(_));



    let mut instruction_data2 = Vec::<u8>::new();
    instruction_data2.push(0u8);
    instruction_data2.push(5u8);
    instruction_data2.push(0u8);
    instruction_data2.push(0u8);
    instruction_data2.push(0u8);

    for byte in name.as_bytes() {
        instruction_data2.push(*byte);
    }

    instruction_data2.push(bump);

    let blockhash2 = rpc_client.get_latest_blockhash().unwrap();


    let mut transaction2 = Transaction::new_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(pda, false),
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
            data: instruction_data2,
        }],
        Some(&payer.pubkey()),
    );
    transaction2.sign(&[&payer], blockhash2);

    assert_matches!(rpc_client.send_and_confirm_transaction(&transaction2), Ok(_));
}
