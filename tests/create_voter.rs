mod common;
/// basic try create voter
#[test]
fn test1_create_voter_basic() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);
    let bob = common::add_account(&mut testvalgen);

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    common::assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
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
}
/// try to reinitialize voter
#[test]
fn test2_create_voter_reinitialize() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);
    let bob = common::add_account(&mut testvalgen);

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    common::assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
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
    common::assert_matches!(
        common::create_voter_transaction(&rpc_client, &initializer, &bob),
        Err(_)
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
}
