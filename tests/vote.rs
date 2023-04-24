use assert_matches::*;
use bpf_program_template::{
    instruction::{
        get_owner_address, get_party_address, get_state_address, get_voter_address,
        string_to_bytearray,
    },
    state::VotesStates,
};
use solana_sdk::signer::Signer;
mod common;

#[test]
fn test_try_vote_basic1() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);
    let bob = common::add_account(&mut testvalgen);
    let alice = common::add_account(&mut testvalgen);
    let party_name = "Alice Party";

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
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_name),
        Ok(_)
    );
    assert_matches!(
        common::create_vote_pos_transaction(&rpc_client, &initializer, &bob, party_name),
        Ok(_)
    );
}

#[test]
fn test_try_vote_basic2() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);
    let bob = common::add_account(&mut testvalgen);
    let alice = common::add_account(&mut testvalgen);
    let ben = common::add_account(&mut testvalgen);
    let party_alice = "Alice Party";
    let party_ben = "Ben Party";

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    let (pda_owner, _owner_bump) = get_owner_address(initializer.pubkey());
    let (pda_state, _state_bump) = get_state_address(pda_owner);
    let (pda_voter, _voter_bump) = get_voter_address(bob.pubkey(), pda_state);

    let name_bytearray = string_to_bytearray(String::from(party_alice));
    let (pda_party_alice, _party_alice_bump) = get_party_address(&name_bytearray, pda_state);

    let name_bytearray = string_to_bytearray(String::from(party_ben));
    let (pda_party_ben, _party_ben_bump) = get_party_address(&name_bytearray, pda_state);

    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
    assert_matches!(
        common::create_voter_transaction(&rpc_client, &initializer, &bob),
        Ok(_)
    );
    let voter_acc = rpc_client.get_account(&pda_voter).unwrap();
    assert_eq!(voter_acc.owner, common::id());
    let voter_data = common::de_account_data(&mut voter_acc.data.as_slice()).unwrap();
    match voter_data {
        bpf_program_template::state::JanecekState::Voter {
            is_initialized,
            author,
            voting_state,
            num_votes,
            pos1,
            pos2,
            neg1,
            bump,
        } => {
            assert!(is_initialized);
            assert_eq!(author, bob.pubkey());
            assert_eq!(voting_state, pda_state);
            assert_eq!(num_votes, VotesStates::Full);
            assert_eq!(pos1, solana_program::system_program::id());
            assert_eq!(pos2, solana_program::system_program::id());
            assert_eq!(neg1, solana_program::system_program::id());
            assert_eq!(bump, _voter_bump);
        }
        _ => {
            assert_eq!(false, true);
        }
    }

    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_alice),
        Ok(_)
    );
    assert_matches!(
        common::create_vote_pos_transaction(&rpc_client, &initializer, &bob, party_alice),
        Ok(_)
    );
    let voter_acc = rpc_client.get_account(&pda_voter).unwrap();
    let voter_data = common::de_account_data(&mut voter_acc.data.as_slice()).unwrap();
    match voter_data {
        bpf_program_template::state::JanecekState::Voter {
            is_initialized,
            author,
            voting_state,
            num_votes,
            pos1,
            pos2,
            neg1,
            bump,
        } => {
            assert!(is_initialized);
            assert_eq!(author, bob.pubkey());
            assert_eq!(voting_state, pda_state);
            assert_eq!(num_votes, VotesStates::OneSpent);
            assert_eq!(pos1, pda_party_alice);
            assert_eq!(pos2, solana_program::system_program::id());
            assert_eq!(neg1, solana_program::system_program::id());
            assert_eq!(bump, _voter_bump);
        }
        _ => {
            assert_eq!(false, true);
        }
    }

    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &ben, party_ben),
        Ok(_)
    );
    assert_matches!(
        common::create_vote_pos_transaction(&rpc_client, &initializer, &bob, party_ben),
        Ok(_)
    );
    let voter_acc = rpc_client.get_account(&pda_voter).unwrap();
    let voter_data = common::de_account_data(&mut voter_acc.data.as_slice()).unwrap();
    match voter_data {
        bpf_program_template::state::JanecekState::Voter {
            is_initialized,
            author,
            voting_state,
            num_votes,
            pos1,
            pos2,
            neg1,
            bump,
        } => {
            assert!(is_initialized);
            assert_eq!(author, bob.pubkey());
            assert_eq!(voting_state, pda_state);
            assert_eq!(num_votes, VotesStates::NoMorePositiveVotes);
            assert_eq!(pos1, pda_party_alice);
            assert_eq!(pos2, pda_party_ben);
            assert_eq!(neg1, solana_program::system_program::id());
            assert_eq!(bump, _voter_bump);
        }
        _ => {
            assert_eq!(false, true);
        }
    }

    assert_matches!(
        common::create_vote_neg_transaction(&rpc_client, &initializer, &bob, party_ben),
        Ok(_)
    );
    let voter_acc = rpc_client.get_account(&pda_voter).unwrap();

    let voter_data = common::de_account_data(&mut voter_acc.data.as_slice()).unwrap();

    match voter_data {
        bpf_program_template::state::JanecekState::Voter {
            is_initialized,
            author,
            voting_state,
            num_votes,
            pos1,
            pos2,
            neg1,
            bump,
        } => {
            assert!(is_initialized);
            assert_eq!(author, bob.pubkey());
            assert_eq!(voting_state, pda_state);
            assert_eq!(num_votes, VotesStates::NoMoreVotes);
            assert_eq!(pos1, pda_party_alice);
            assert_eq!(pos2, pda_party_ben);
            assert_eq!(neg1, pda_party_ben);
            assert_eq!(bump, _voter_bump);
        }
        _ => {
            assert_eq!(false, true);
        }
    }
}

#[test]
fn test_try_vote_basic3() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);
    let bob = common::add_account(&mut testvalgen);
    let alice = common::add_account(&mut testvalgen);
    let party_name = "ラウトは難しいです！";

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
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_name),
        Ok(_)
    );
    assert_matches!(
        common::create_vote_pos_transaction(&rpc_client, &initializer, &bob, party_name),
        Ok(_)
    );
}

#[test]
fn test_try_vote_basic4() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);
    let bob = common::add_account(&mut testvalgen);
    let alice = common::add_account(&mut testvalgen);
    let party_name = "ラウトは難しいです！";

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    assert_matches!(
        common::create_voter_transaction(&rpc_client, &initializer, &bob),
        Err(_)
    );
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_name),
        Err(_)
    );
    assert_matches!(
        common::create_vote_pos_transaction(&rpc_client, &initializer, &bob, party_name),
        Err(_)
    );
    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
    assert_matches!(
        common::create_voter_transaction(&rpc_client, &initializer, &bob),
        Ok(_)
    );
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_name),
        Ok(_)
    );
    assert_matches!(
        common::create_vote_pos_transaction(&rpc_client, &initializer, &bob, party_name),
        Ok(_)
    );
}
