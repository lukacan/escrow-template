use bpf_program_template::instruction::initialize;
// #![cfg(feature = "test-sbf")]
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_sdk::{account::AccountSharedData, declare_id, signature::Keypair};

use {
    solana_sdk::{signature::Signer, transaction::Transaction},
    solana_validator::test_validator::*,
};

declare_id!("Fnambs3f1XXoMmAVc94bf8t6JDAxmVkXz85XU4v2edph");

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
fn test_basic() {
    solana_logger::setup_with_default("solana_program_runtime=debug");
    solana_logger::setup_with_default("solana_runtime::message=debug");
    let mut testvalgen = TestValidatorGenesis::default();

    let initializer = add_account(&mut testvalgen);

    let (test_validator, _payer) = testvalgen
        .add_program("target/deploy/bpf_program_template", id())
        .start();
    let rpc_client = test_validator.get_rpc_client();

    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[initialize(initializer.pubkey())],
        Some(&initializer.pubkey()),
    );

    transaction.sign(&[&initializer], blockhash);
    rpc_client
        .send_and_confirm_transaction(&transaction)
        .unwrap();
}
