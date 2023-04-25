use bpf_program_template::instruction::get_owner_address;
// #![cfg(feature = "test-sbf")]
use assert_matches::*;
use base64::{engine::general_purpose, Engine as _};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_sdk::{account::AccountSharedData, signer::Signer};

mod common;

/// basic initialize
#[test]
fn test1_initialize_basic() {
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
fn test2_initialize_non_zero_balance() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);

    let (pda_owner, _bump_wner) = get_owner_address(initializer.pubkey());

    let pda_owner_acc = AccountSharedData::new(
        10000, // a few, but not 0, to invoke transfer
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
    common::compare_voting_owner_data(&rpc_client, &initializer);
    common::compare_voting_state_data(&rpc_client, &initializer);
}

/// REINITIALIZE
#[test]
fn test3_initialize_reinitialize() {
    let mut testvalgen = common::init_env();

    let initializer = common::add_account(&mut testvalgen);
    let initializer2 = common::add_account(&mut testvalgen);

    // create pda account with non-zero balance, to check reinitialization also of this
    let (pda_owner, _bump_wner) = get_owner_address(initializer2.pubkey());

    let pda_owner_acc = AccountSharedData::new(
        10000, // a few, but not 0, to invoke transfer
        0,
        &solana_program::system_program::id(),
    );
    testvalgen.add_account(pda_owner, pda_owner_acc);

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    // initializer pda transactions and checks
    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
    common::compare_voting_owner_data(&rpc_client, &initializer);
    common::compare_voting_state_data(&rpc_client, &initializer);
    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Err(_)
    );
    common::compare_voting_owner_data(&rpc_client, &initializer);
    common::compare_voting_state_data(&rpc_client, &initializer);

    // initializer2 pda transactions and checks
    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer2),
        Ok(_)
    );
    common::compare_voting_owner_data(&rpc_client, &initializer2);
    common::compare_voting_state_data(&rpc_client, &initializer2);
    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer2),
        Err(_)
    );
    common::compare_voting_owner_data(&rpc_client, &initializer2);
    common::compare_voting_state_data(&rpc_client, &initializer2);
}

// initialize with account with sus data
#[test]
fn test4_initialize_spoofed_data() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);

    let (pda_owner, _bump_wner) = get_owner_address(initializer.pubkey());
    testvalgen.add_account_with_base64_data(
        pda_owner,
        LAMPORTS_PER_SOL * 2,
        solana_program::system_program::id(),
        &general_purpose::STANDARD.encode([1u8, 0u8])[..],
    );
    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Err(_)
    );
}
