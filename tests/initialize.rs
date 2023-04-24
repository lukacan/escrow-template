use bpf_program_template::instruction::get_owner_address;
// #![cfg(feature = "test-sbf")]
use assert_matches::*;
use base64::{engine::general_purpose, Engine as _};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_sdk::{account::AccountSharedData, signer::Signer};

mod common;

/// basic initialize call
#[test]
fn test_initialize_basic1() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
    common::compare_voting_owner_data(&rpc_client, &initializer);
    common::compare_voting_state_data(&rpc_client, &initializer);
}
/// basic initialize call with acc whose balance is not zero
#[test]
fn test_initialize_basic2() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);

    let (pda_owner, _bump_wner) = get_owner_address(initializer.pubkey());

    let pda_owner_acc = AccountSharedData::new(
        LAMPORTS_PER_SOL * 2,
        0,
        &solana_program::system_program::id(),
    );
    testvalgen.add_account(pda_owner, pda_owner_acc);

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
}

/// test that tries to reinitialize
#[test]
fn test_initialize_basic3() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Err(_)
    );
}

#[test]
fn test_initialize_basic4() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);

    let (pda_owner, _bump_wner) = get_owner_address(initializer.pubkey());
    testvalgen.add_account_with_base64_data(
        pda_owner,
        LAMPORTS_PER_SOL * 2,
        solana_program::system_program::id(),
        &general_purpose::STANDARD.encode([0u8, 0u8])[..],
    );
    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Err(_)
    );
}
