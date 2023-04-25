use assert_matches::*;
use bpf_program_template::{
    instruction::{get_owner_address, get_party_address, get_state_address, string_to_bytearray},
    state::VotesStates,
};
use solana_sdk::signer::Signer;
mod common;

///complex vote test with vote positive, reinitializations etc.
#[test]
fn test1_try_vote_complex() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);

    let bob = common::add_account(&mut testvalgen); // party owner 1
    let alice = common::add_account(&mut testvalgen); // party owner 2
    let michael = common::add_account(&mut testvalgen); // party owner 3

    let party_alice = "ラウトは難しいです！";
    let party_ben = "ウトは難しいです！";
    let party_michael = "Andrej Babis";

    let ben = common::add_account(&mut testvalgen); // this is voter

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    let (pda_owner, _owner_bump) = get_owner_address(initializer.pubkey());
    let (pda_state, _state_bump) = get_state_address(pda_owner);

    let name_bytearray = string_to_bytearray(String::from(party_alice));
    let (pda_party_alice, _party_alice_bump) = get_party_address(&name_bytearray, pda_state);

    let name_bytearray = string_to_bytearray(String::from(party_ben));
    let (pda_party_ben, _party_ben_bump) = get_party_address(&name_bytearray, pda_state);

    let name_bytearray = string_to_bytearray(String::from(party_michael));
    let (pda_party_michael, _party_michael_bump) = get_party_address(&name_bytearray, pda_state);

    // try create in unintialized contetx
    assert_matches!(
        common::create_voter_transaction(&rpc_client, &initializer, &bob),
        Err(_)
    );
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_alice),
        Err(_)
    );

    // initialize context
    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
    common::compare_voting_owner_data(&rpc_client, &initializer);
    common::compare_voting_state_data(&rpc_client, &initializer);

    // create voter
    assert_matches!(
        common::create_voter_transaction(&rpc_client, &initializer, &bob),
        Ok(_)
    );
    common::compare_voter_data(
        &rpc_client,
        &initializer,
        &bob,
        VotesStates::Full,
        solana_program::system_program::id(),
        solana_program::system_program::id(),
        solana_program::system_program::id(),
    );

    // create parties
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_alice),
        Ok(_)
    );
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &ben, party_ben),
        Ok(_)
    );
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &michael, party_michael),
        Ok(_)
    );
    common::compare_party_data(&rpc_client, &initializer, &alice, party_alice, 0);
    common::compare_party_data(&rpc_client, &initializer, &ben, party_ben, 0);
    common::compare_party_data(&rpc_client, &initializer, &michael, party_michael, 0);

    // vote positive
    assert_matches!(
        common::create_vote_pos_transaction(&rpc_client, &initializer, &bob, party_alice),
        Ok(_)
    );
    common::compare_party_data(&rpc_client, &initializer, &alice, party_alice, 1);
    common::compare_voter_data(
        &rpc_client,
        &initializer,
        &bob,
        VotesStates::OneSpent,
        pda_party_alice,
        solana_program::system_program::id(),
        solana_program::system_program::id(),
    );

    // try to reinitialize alice party
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_alice),
        Err(_)
    );
    common::compare_party_data(&rpc_client, &initializer, &alice, party_alice, 1);

    // vote negative withour spending 2 pos votes
    assert_matches!(
        common::create_vote_neg_transaction(&rpc_client, &initializer, &bob, party_ben),
        Err(_)
    );
    common::compare_voter_data(
        &rpc_client,
        &initializer,
        &bob,
        VotesStates::OneSpent,
        pda_party_alice,
        solana_program::system_program::id(),
        solana_program::system_program::id(),
    );

    // vote positive for party ben
    assert_matches!(
        common::create_vote_pos_transaction(&rpc_client, &initializer, &bob, party_ben),
        Ok(_)
    );
    common::compare_party_data(&rpc_client, &initializer, &ben, party_ben, 1);
    common::compare_voter_data(
        &rpc_client,
        &initializer,
        &bob,
        VotesStates::NoMorePositiveVotes,
        pda_party_alice,
        pda_party_ben,
        solana_program::system_program::id(),
    );

    // vote negative for party michael
    assert_matches!(
        common::create_vote_neg_transaction(&rpc_client, &initializer, &bob, party_michael),
        Ok(_)
    );
    common::compare_party_data(&rpc_client, &initializer, &ben, party_ben, 1);
    common::compare_party_data(&rpc_client, &initializer, &alice, party_alice, 1);
    common::compare_party_data(&rpc_client, &initializer, &michael, party_michael, -1);

    common::compare_voter_data(
        &rpc_client,
        &initializer,
        &bob,
        VotesStates::NoMoreVotes,
        pda_party_alice,
        pda_party_ben,
        pda_party_michael,
    );

    // reinitialize everything
    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Err(_)
    );
    common::compare_voting_owner_data(&rpc_client, &initializer);
    common::compare_voting_state_data(&rpc_client, &initializer);

    assert_matches!(
        common::create_voter_transaction(&rpc_client, &initializer, &bob),
        Err(_)
    );
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_alice),
        Err(_)
    );
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &ben, party_ben),
        Err(_)
    );
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &michael, party_michael),
        Err(_)
    );
    common::compare_party_data(&rpc_client, &initializer, &alice, party_alice, 1);
    common::compare_party_data(&rpc_client, &initializer, &ben, party_ben, 1);
    common::compare_party_data(&rpc_client, &initializer, &michael, party_michael, -1);
    common::compare_voter_data(
        &rpc_client,
        &initializer,
        &bob,
        VotesStates::NoMoreVotes,
        pda_party_alice,
        pda_party_ben,
        pda_party_michael,
    );
}
