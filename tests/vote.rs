use solana_sdk::signer::Signer;

mod common;

///complex vote test with vote positive, reinitializations etc.
#[test]
fn test1_try_vote_complex() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);

    let ben = common::add_account(&mut testvalgen); // party owner 1
    let alice = common::add_account(&mut testvalgen); // party owner 2
    let michael = common::add_account(&mut testvalgen); // party owner 3

    let party_alice = "ラウトは難しいです！";
    let party_ben = "ウトは難しいです！";
    let party_michael = "Andrej Babis";

    let bob = common::add_account(&mut testvalgen); // this is voter

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    let (pda_owner, _owner_bump) =
        common::get_owner_address(solana_sdk::signer::Signer::pubkey(&initializer));
    let (pda_state, _state_bump) = common::get_state_address(pda_owner);

    let name_bytearray = common::string_to_bytearray(String::from(party_alice));
    let (pda_party_alice, _party_alice_bump) =
        common::get_party_address(&name_bytearray, pda_state);

    let name_bytearray = common::string_to_bytearray(String::from(party_ben));
    let (pda_party_ben, _party_ben_bump) = common::get_party_address(&name_bytearray, pda_state);

    let name_bytearray = common::string_to_bytearray(String::from(party_michael));
    let (pda_party_michael, _party_michael_bump) =
        common::get_party_address(&name_bytearray, pda_state);

    // try create in unintialized contetx
    common::assert_matches!(
        common::create_voter_transaction(&rpc_client, &initializer, &bob),
        Err(_)
    );
    common::assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_alice),
        Err(_)
    );

    // initialize context
    common::assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
    common::compare_voting_owner_data(&rpc_client, &initializer);
    common::compare_voting_state_data(&rpc_client, &initializer);

    // create voter
    common::assert_matches!(
        common::create_voter_transaction(&rpc_client, &initializer, &bob),
        Ok(_)
    );
    common::compare_voter_data(
        &rpc_client,
        &initializer,
        &bob,
        common::VotesStates::Full,
        solana_program::system_program::id(),
        solana_program::system_program::id(),
        solana_program::system_program::id(),
    );

    // create parties
    common::assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_alice),
        Ok(_)
    );
    common::assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &ben, party_ben),
        Ok(_)
    );
    common::assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &michael, party_michael),
        Ok(_)
    );
    common::compare_party_data(&rpc_client, &initializer, &alice, party_alice, 0);
    common::compare_party_data(&rpc_client, &initializer, &ben, party_ben, 0);
    common::compare_party_data(&rpc_client, &initializer, &michael, party_michael, 0);

    // vote positive
    common::assert_matches!(
        common::create_vote_pos_transaction(&rpc_client, &initializer, &bob, party_alice),
        Ok(_)
    );
    common::compare_party_data(&rpc_client, &initializer, &alice, party_alice, 1);
    common::compare_voter_data(
        &rpc_client,
        &initializer,
        &bob,
        common::VotesStates::OneSpent,
        pda_party_alice,
        solana_program::system_program::id(),
        solana_program::system_program::id(),
    );

    // try to reinitialize alice party
    common::assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_alice),
        Err(_)
    );
    common::compare_party_data(&rpc_client, &initializer, &alice, party_alice, 1);

    // vote negative withour spending 2 pos votes
    common::assert_matches!(
        common::create_vote_neg_transaction(&rpc_client, &initializer, &bob, party_ben),
        Err(_)
    );
    common::compare_voter_data(
        &rpc_client,
        &initializer,
        &bob,
        common::VotesStates::OneSpent,
        pda_party_alice,
        solana_program::system_program::id(),
        solana_program::system_program::id(),
    );

    // vote positive for party ben
    common::assert_matches!(
        common::create_vote_pos_transaction(&rpc_client, &initializer, &bob, party_ben),
        Ok(_)
    );
    common::compare_party_data(&rpc_client, &initializer, &ben, party_ben, 1);
    common::compare_voter_data(
        &rpc_client,
        &initializer,
        &bob,
        common::VotesStates::NoMorePositiveVotes,
        pda_party_alice,
        pda_party_ben,
        solana_program::system_program::id(),
    );

    // vote negative for party michael
    common::assert_matches!(
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
        common::VotesStates::NoMoreVotes,
        pda_party_alice,
        pda_party_ben,
        pda_party_michael,
    );

    // reinitialize everything
    common::assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Err(_)
    );
    common::compare_voting_owner_data(&rpc_client, &initializer);
    common::compare_voting_state_data(&rpc_client, &initializer);

    common::assert_matches!(
        common::create_voter_transaction(&rpc_client, &initializer, &bob),
        Err(_)
    );
    common::assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_alice),
        Err(_)
    );
    common::assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &ben, party_ben),
        Err(_)
    );
    common::assert_matches!(
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
        common::VotesStates::NoMoreVotes,
        pda_party_alice,
        pda_party_ben,
        pda_party_michael,
    );
}

#[test]
fn test2_overflow_votes() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);

    let alice = common::add_account(&mut testvalgen); // party owner 2

    let party_alice = "ラウトは難しいです！";

    let bob = common::add_account(&mut testvalgen); // this is voter

    let (pda_owner, _owner_bump) =
        common::get_owner_address(solana_sdk::signer::Signer::pubkey(&initializer));
    let (pda_state, _state_bump) = common::get_state_address(pda_owner);

    let name_bytearray = common::string_to_bytearray(String::from(party_alice));
    let (pda_party_alice, _party_alice_bump) =
        common::get_party_address(&name_bytearray, pda_state);

    let mut buffer = [0; common::JanecekState::LEN_PARTY];
    common::se_account(
        common::JanecekState::Party {
            is_initialized: true,
            author: alice.pubkey(),
            voting_state: pda_state,
            created: 0,
            name: name_bytearray,
            votes: common::MAX,
            bump: _party_alice_bump,
        },
        &mut buffer,
    );

    testvalgen.add_account_with_base64_data(
        pda_party_alice,
        common::LAMPORTS_PER_SOL * 2,
        common::id(),
        &base64::Engine::encode(&common::general_purpose::STANDARD, buffer)[..],
    );

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    // initialize context
    common::assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
    common::compare_voting_owner_data(&rpc_client, &initializer);
    common::compare_voting_state_data(&rpc_client, &initializer);
    common::compare_party_data(&rpc_client, &initializer, &alice, party_alice, common::MAX);

    // create voter
    common::assert_matches!(
        common::create_voter_transaction(&rpc_client, &initializer, &bob),
        Ok(_)
    );
    // compare voter data
    common::compare_voter_data(
        &rpc_client,
        &initializer,
        &bob,
        common::VotesStates::Full,
        solana_program::system_program::id(),
        solana_program::system_program::id(),
        solana_program::system_program::id(),
    );
    // vote positive for overflow
    common::assert_matches!(
        common::create_vote_pos_transaction(&rpc_client, &initializer, &bob, party_alice),
        Err(_)
    );

    common::compare_party_data(&rpc_client, &initializer, &alice, party_alice, common::MAX);

    common::compare_voter_data(
        &rpc_client,
        &initializer,
        &bob,
        common::VotesStates::Full,
        solana_program::system_program::id(),
        solana_program::system_program::id(),
        solana_program::system_program::id(),
    );
}
