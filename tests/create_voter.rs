use assert_matches::*;
use bpf_program_template::{
    instruction::{get_owner_address, get_state_address, get_voter_address},
    state::{JanecekState, VotesStates},
};
use solana_sdk::signer::Signer;

mod common;
/// basic try create voter
#[test]
fn test_create_voter_basic1() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);
    let bob = common::add_account(&mut testvalgen);

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
    assert_matches!(
        common::create_voter_transaction(&rpc_client, &initializer, &bob),
        Ok(_)
    );

    let (pda_owner, _owner_bump) = get_owner_address(initializer.pubkey());
    let (pda_state, _state_bump) = get_state_address(pda_owner);
    let (pda_voter, _voter_bump) = get_voter_address(bob.pubkey(), pda_state);

    let voter_acc = rpc_client.get_account(&pda_voter).unwrap();

    assert_eq!(voter_acc.owner, common::id());

    let voter_data = common::de_account_data(&mut voter_acc.data.as_slice()).unwrap();

    assert_eq!(
        voter_data,
        JanecekState::Voter {
            is_initialized: true,
            author: bob.pubkey(),
            voting_state: pda_state,
            num_votes: VotesStates::Full,
            pos1: solana_program::system_program::id(),
            pos2: solana_program::system_program::id(),
            neg1: solana_program::system_program::id(),
            bump: _voter_bump
        }
    );
}
/// try to reinitialize voter
#[test]
fn test_create_voter_basic2() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);
    let bob = common::add_account(&mut testvalgen);

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
    assert_matches!(
        common::create_voter_transaction(&rpc_client, &initializer, &bob),
        Ok(_)
    );
    assert_matches!(
        common::create_voter_transaction(&rpc_client, &initializer, &bob),
        Err(_)
    );
}
