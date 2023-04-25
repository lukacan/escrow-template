use core::time;
use std::thread;

use bpf_program_template::{
    instruction::{get_owner_address, get_party_address, get_state_address},
    state::JanecekState,
};
// #![cfg(feature = "test-sbf")]
use solana_program::{
    instruction::{AccountMeta, Instruction},
    system_program,
};
use solana_sdk::{signer::Signer, transaction::Transaction};

use assert_matches::*;

mod common;
/// basic test to try create party
#[test]
fn test1_create_party_basic() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);
    let alice = common::add_account(&mut testvalgen);
    let party_name = "ラウトは難しいです！"; // chars 10 , but bytes 30

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_name,),
        Ok(_)
    );
    common::compare_party_data(&rpc_client, &initializer, &alice, party_name, 0);
}
/// REINITIALIZE
#[test]
fn test2_create_party_reinitialize() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);
    let alice = common::add_account(&mut testvalgen);
    let party_name = "Alice Party";

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_name,),
        Ok(_)
    );
    common::compare_party_data(&rpc_client, &initializer, &alice, party_name, 0);

    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_name,),
        Err(_)
    );
    common::compare_party_data(&rpc_client, &initializer, &alice, party_name, 0);
}

/// test that creates party with special name
#[test]
fn test3_create_party_special_name() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);
    let alice = common::add_account(&mut testvalgen);

    // Max seed length in bytes is 32
    let party_name = "ラウトは難しいです!!!!";

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_name,),
        Ok(_)
    );
    common::compare_party_data(&rpc_client, &initializer, &alice, party_name, 0);
}

/// test longer names
#[test]
#[should_panic]
fn test4_create_party_long_name() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);
    let alice = common::add_account(&mut testvalgen);

    // Should panic as the seed length is exceeded
    let party_name = "ラウトは難しいです!!!!!";

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_name,),
        Err(_)
    );
}
#[test]
fn test5_create_party_spoofed_data() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);
    let alice = common::add_account(&mut testvalgen);

    let party_name = String::from("Alice Party");
    let mut name_bytearray: [u8; JanecekState::NAME_LENGTH] = [0u8; JanecekState::NAME_LENGTH];
    name_bytearray[..party_name.len()].copy_from_slice(party_name.into_bytes().as_slice());

    // derive PDA from normal name
    let (owner, bump_owner) = get_owner_address(initializer.pubkey());
    let (state, bump_state) = get_state_address(owner);
    let (party, _bump_party) = get_party_address(&name_bytearray, state);

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );

    // but send wrong long name as instruction data
    let party_name = String::from(
        "Alice PartyVery long long long long long long long long long long long name for alice party",
    );

    let mut data = vec![1u8, bump_owner, bump_state];
    for byte in party_name.as_bytes() {
        data.push(*byte);
    }

    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: common::id(),
            accounts: vec![
                AccountMeta::new(alice.pubkey(), true),
                AccountMeta::new_readonly(initializer.pubkey(), true),
                AccountMeta::new_readonly(owner, false),
                AccountMeta::new_readonly(state, false),
                AccountMeta::new(party, false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
            data,
        }],
        Some(&alice.pubkey()),
    );

    transaction.sign(&[&alice, &initializer], blockhash);
    assert_matches!(
        rpc_client.send_and_confirm_transaction(&transaction),
        Err(_)
    );
}
#[test]
#[should_panic]
fn test6_create_party_api_panic() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);
    let alice = common::add_account(&mut testvalgen);

    let party_name =
        "Very long long long long long long long long long long long name for alice party";
    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_name),
        Ok(_)
    );
}

/// check if party can be created after voting ended
#[test]
#[ignore = "voting ended test, to test this, voting time has to be decreased to 5s"]
fn test7_create_party_after_deadline() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);
    let alice = common::add_account(&mut testvalgen);
    let party_name = "ラウトは難しいです！"; // chars 10 , but bytes 30

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );

    // sleep 10 seconds
    let ten_millis = time::Duration::from_millis(10000);
    thread::sleep(ten_millis);

    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_name,),
        Err(_)
    );
}

#[test]
#[should_panic]
fn test8_create_party_without_initialize() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);
    let alice = common::add_account(&mut testvalgen);
    let party_name = "ラウトは難しいです！"; // chars 10 , but bytes 30

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_name,),
        Err(_)
    );
    common::compare_party_data(&rpc_client, &initializer, &alice, party_name, 0);
}

#[test]
#[should_panic]
fn test9_create_party_same_name() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);
    let alice = common::add_account(&mut testvalgen);
    let bob = common::add_account(&mut testvalgen);

    let party_name_alice = "ラウトは難しいです！"; // chars 10 , but bytes 30
    let party_name_bob = "ラウトは難しいです！"; // chars 10 , but bytes 30

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_name_alice),
        Ok(_)
    );
    common::compare_party_data(&rpc_client, &initializer, &alice, party_name_alice, 0);
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &bob, party_name_bob),
        Err(_)
    );
    common::compare_party_data(&rpc_client, &initializer, &bob, party_name_bob, 0);
}
