use bpf_program_template::{
    instruction::{get_owner_address, get_state_address},
    state::JanecekState,
};
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

    let (pda_owner, _owner_bump) = get_owner_address(initializer.pubkey());
    let (pda_state, _state_bump) = get_state_address(pda_owner);

    let voting_owner_acc = rpc_client.get_account(&pda_owner).unwrap();
    let voting_state_acc = rpc_client.get_account(&pda_state).unwrap();

    assert_eq!(voting_owner_acc.owner, common::id());
    assert_eq!(voting_state_acc.owner, common::id());

    let voting_owner_data = common::de_account_data(&mut voting_owner_acc.data.as_slice()).unwrap();
    let voting_state_data = common::de_account_data(&mut voting_state_acc.data.as_slice()).unwrap();

    match voting_owner_data {
        bpf_program_template::state::JanecekState::VotingOwner {
            is_initialized,
            author,
            voting_state,
            bump,
        } => {
            assert!(is_initialized);
            assert_eq!(author, initializer.pubkey());
            assert_eq!(voting_state, pda_state);
            assert_eq!(bump, _owner_bump);
        }
        _ => {
            assert_eq!(false, true);
        }
    }

    match voting_state_data {
        bpf_program_template::state::JanecekState::VotingState {
            is_initialized,
            voting_owner,
            voting_started,
            voting_ends,
            bump,
        } => {
            assert!(is_initialized);
            assert_eq!(voting_owner, pda_owner);
            assert_eq!(voting_ends - voting_started, JanecekState::VOTING_LENGTH);
            assert_eq!(bump, _state_bump);
        }
        _ => {
            assert_eq!(false, true);
        }
    }
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
