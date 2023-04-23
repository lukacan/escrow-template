use assert_matches::*;
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

    assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
    assert_matches!(
        common::create_voter_transaction(&rpc_client, &initializer, &bob),
        Ok(_)
    );
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &alice, party_alice),
        Ok(_)
    );
    assert_matches!(
        common::create_vote_pos_transaction(&rpc_client, &initializer, &bob, party_alice),
        Ok(_)
    );
    assert_matches!(
        common::create_party_transaction(&rpc_client, &initializer, &ben, party_ben),
        Ok(_)
    );
    assert_matches!(
        common::create_vote_pos_transaction(&rpc_client, &initializer, &bob, party_ben),
        Ok(_)
    );
    assert_matches!(
        common::create_vote_neg_transaction(&rpc_client, &initializer, &bob, party_ben),
        Ok(_)
    );
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
