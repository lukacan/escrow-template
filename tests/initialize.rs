mod common;

/// basic initialize
#[test]
fn test1_initialize_basic() {
    let mut testvalgen = common::init_env();
    let initializer = common::add_account(&mut testvalgen);

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    common::assert_matches!(
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

    let (pda_owner, _bump_wner) =
        common::get_owner_address(solana_sdk::signer::Signer::pubkey(&initializer));

    let pda_owner_acc = common::AccountSharedData::new(
        10000, // a few, but not 0, to invoke transfer
        0,
        &solana_program::system_program::id(),
    );
    testvalgen.add_account(pda_owner, pda_owner_acc);

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    common::assert_matches!(
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
    let (pda_owner, _bump_wner) =
        common::get_owner_address(solana_sdk::signer::Signer::pubkey(&initializer2));

    let pda_owner_acc = common::AccountSharedData::new(
        10000, // a few, but not 0, to invoke transfer
        0,
        &solana_program::system_program::id(),
    );
    testvalgen.add_account(pda_owner, pda_owner_acc);

    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    // initializer pda transactions and checks
    common::assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Ok(_)
    );
    common::compare_voting_owner_data(&rpc_client, &initializer);
    common::compare_voting_state_data(&rpc_client, &initializer);
    common::assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Err(_)
    );
    common::compare_voting_owner_data(&rpc_client, &initializer);
    common::compare_voting_state_data(&rpc_client, &initializer);

    // initializer2 pda transactions and checks
    common::assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer2),
        Ok(_)
    );
    common::compare_voting_owner_data(&rpc_client, &initializer2);
    common::compare_voting_state_data(&rpc_client, &initializer2);
    common::assert_matches!(
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

    let (pda_owner, _bump_wner) =
        common::get_owner_address(solana_sdk::signer::Signer::pubkey(&initializer));
    testvalgen.add_account_with_base64_data(
        pda_owner,
        common::LAMPORTS_PER_SOL * 2,
        solana_program::system_program::id(),
        &base64::Engine::encode(&common::general_purpose::STANDARD, [1u8, 0u8])[..],
    );
    let (test_validator, _payer) = testvalgen.start();
    let rpc_client = test_validator.get_rpc_client();

    common::assert_matches!(
        common::initialize_transaction(&rpc_client, &initializer),
        Err(_)
    );
}
