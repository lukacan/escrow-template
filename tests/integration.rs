//#![cfg(feature = "test-bpf")]
use solana_client::client_error::{ClientError, ClientErrorKind};
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::InstructionError;
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_sdk::transaction::TransactionError;
use solana_sdk::{
    account::AccountSharedData,
    declare_id,
    signature::{Keypair, Signature},
};

use {
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_sdk::{signature::Signer, transaction::Transaction},
    solana_validator::test_validator::*,
};

use assert_matches::assert_matches;

declare_id!("Fnambs3f1XXoMmAVc94bf8t6JDAxmVkXz85XU4v2edph");
fn initialize(rpc_client: &RpcClient, initializer: &Keypair, expected_err:u32) {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let (pda_owner, bump_owner) =
        Pubkey::find_program_address(&[b"voting_owner", initializer.pubkey().as_ref()], &id());

    let (pda_state, bump_state) =
        Pubkey::find_program_address(&[b"voting_state", pda_owner.as_ref()], &id());

    let instruction_data = vec![0u8, bump_owner, bump_state];

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(initializer.pubkey(), true), // initializer
                AccountMeta::new(pda_owner, false),           // voting owner
                AccountMeta::new(pda_state, false),           // voting state
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
            data: instruction_data,
        }],
        Some(&initializer.pubkey()),
    );
    transaction.sign(&[initializer], blockhash);
    compare_error(rpc_client.send_and_confirm_transaction(&transaction), expected_err);
    //rpc_client.send_and_confirm_transaction(&transaction)
}

fn create_party(
    rpc_client: &RpcClient,
    initializer: &Keypair,
    person: &Keypair,
    party_name: &String,
    expected_err:u32,
) {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let (pda_owner, bump_owner) =
        Pubkey::find_program_address(&[b"voting_owner", initializer.pubkey().as_ref()], &id());

    let (pda_state, bump_state) =
        Pubkey::find_program_address(&[b"voting_state", pda_owner.as_ref()], &id());

    let (pda_party, bump_party) =
        Pubkey::find_program_address(&[party_name.as_bytes(), pda_state.as_ref()], &id());

    let mut instruction_data = vec![
        1u8,
        bump_owner,
        bump_state,
        bump_party,
        party_name.chars().count() as u8,
        0u8,
        0u8,
        0u8,
    ];
    for byte in party_name.as_bytes() {
        instruction_data.push(*byte);
    }

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(person.pubkey(), true), // persone that wants to create party
                AccountMeta::new_readonly(initializer.pubkey(), true), // owner
                AccountMeta::new_readonly(pda_owner, false), // voting owner
                AccountMeta::new_readonly(pda_state, false), // voting state
                AccountMeta::new(pda_party, false),      // party
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
            data: instruction_data,
        }],
        Some(&person.pubkey()),
    );
    transaction.sign(&[person, initializer], blockhash);

    //rpc_client.send_and_confirm_transaction(&transaction)
    compare_error(rpc_client.send_and_confirm_transaction(&transaction), expected_err);
}

fn create_voter(rpc_client: &RpcClient, initializer: &Pubkey, voter: &Keypair,expected_err:u32) {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let (pda_owner, bump_owner) =
        Pubkey::find_program_address(&[b"voting_owner", initializer.as_ref()], &id());

    let (pda_state, bump_state) =
        Pubkey::find_program_address(&[b"voting_state", pda_owner.as_ref()], &id());

    let (pda_voter, bump_voter) = Pubkey::find_program_address(
        &[b"new_voter", voter.pubkey().as_ref(), pda_state.as_ref()],
        &id(),
    );

    let instruction_data = vec![2u8, bump_owner, bump_state, bump_voter];

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(voter.pubkey(), true), // persone that wants to be voter
                AccountMeta::new_readonly(*initializer, false), // owner
                AccountMeta::new_readonly(pda_owner, false), // voting owner
                AccountMeta::new_readonly(pda_state, false), // voting state
                AccountMeta::new(pda_voter, false),     // voter
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
            data: instruction_data,
        }],
        Some(&voter.pubkey()),
    );
    transaction.sign(&[voter], blockhash);

    //rpc_client.send_and_confirm_transaction(&transaction)
    compare_error(rpc_client.send_and_confirm_transaction(&transaction), expected_err);
}

fn vote_positive(
    rpc_client: &RpcClient,
    initializer: &Pubkey,
    voter: &Keypair,
    party_name: &String,
    expected_err:u32,
) {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let (pda_owner, bump_owner) =
        Pubkey::find_program_address(&[b"voting_owner", initializer.as_ref()], &id());

    let (pda_state, bump_state) =
        Pubkey::find_program_address(&[b"voting_state", pda_owner.as_ref()], &id());

    let (pda_voter, bump_voter) = Pubkey::find_program_address(
        &[b"new_voter", voter.pubkey().as_ref(), pda_state.as_ref()],
        &id(),
    );
    let (pda_party, bump_party) =
        Pubkey::find_program_address(&[party_name.as_bytes(), pda_state.as_ref()], &id());

    let mut instruction_data = vec![
        3u8,
        bump_owner,
        bump_state,
        bump_voter,
        bump_party,
        party_name.chars().count() as u8,
        0u8,
        0u8,
        0u8,
    ];
    for byte in party_name.as_bytes() {
        instruction_data.push(*byte);
    }

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(voter.pubkey(), true), // person that wants to be voter
                AccountMeta::new_readonly(*initializer, false), // owner
                AccountMeta::new_readonly(pda_owner, false), // voting owner
                AccountMeta::new_readonly(pda_state, false), // voting state
                AccountMeta::new(pda_voter, false),     // voter
                AccountMeta::new(pda_party, false),     // party
            ],
            data: instruction_data,
        }],
        Some(&voter.pubkey()),
    );
    transaction.sign(&[voter], blockhash);
    compare_error(rpc_client.send_and_confirm_transaction(&transaction), expected_err);
    //rpc_client.send_and_confirm_transaction(&transaction)
}

fn vote_negative(
    rpc_client: &RpcClient,
    initializer: &Pubkey,
    voter: &Keypair,
    party_name: &String,
    expected_err:u32,
) {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let (pda_owner, bump_owner) =
        Pubkey::find_program_address(&[b"voting_owner", initializer.as_ref()], &id());

    let (pda_state, bump_state) =
        Pubkey::find_program_address(&[b"voting_state", pda_owner.as_ref()], &id());

    let (pda_voter, bump_voter) = Pubkey::find_program_address(
        &[b"new_voter", voter.pubkey().as_ref(), pda_state.as_ref()],
        &id(),
    );
    let (pda_party, bump_party) =
        Pubkey::find_program_address(&[party_name.as_bytes(), pda_state.as_ref()], &id());

    let mut instruction_data = vec![
        4u8,
        bump_owner,
        bump_state,
        bump_voter,
        bump_party,
        party_name.chars().count() as u8,
        0u8,
        0u8,
        0u8,
    ];
    for byte in party_name.as_bytes() {
        instruction_data.push(*byte);
    }

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(voter.pubkey(), true), // persone that wants to be voter
                AccountMeta::new_readonly(*initializer, false), // owner
                AccountMeta::new_readonly(pda_owner, false), // voting owner
                AccountMeta::new_readonly(pda_state, false), // voting state
                AccountMeta::new(pda_voter, false),     // voter
                AccountMeta::new(pda_party, false),     // party
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
            data: instruction_data,
        }],
        Some(&voter.pubkey()),
    );
    transaction.sign(&[voter], blockhash);
    compare_error(rpc_client.send_and_confirm_transaction(&transaction), expected_err);
    //rpc_client.send_and_confirm_transaction(&transaction)
}

fn add_account(testvalgen: &mut TestValidatorGenesis) -> Keypair {
    let alice = Keypair::new();
    let account = AccountSharedData::new(
        LAMPORTS_PER_SOL * 2,
        0,
        &solana_program::system_program::id(),
    );
    testvalgen.add_account(alice.pubkey(), account);
    return alice;
}

fn compare_error(result: Result<Signature, ClientError>, expected_value: u32) {
    match result.err() {
        Some(error) => match error.kind() {
            ClientErrorKind::RpcError(_) => match error.get_transaction_error().unwrap() {
                TransactionError::InstructionError(_x, y) => match y {
                    InstructionError::Custom(z) => {
                        assert_eq!(z, expected_value)
                    }
                    _ => {
                        println!("Failed4 with {}", y);
                        assert_eq!(false, true);
                    }
                },
                _ => {
                    println!("Failed3");
                    assert_eq!(false, true);
                }
            },
            _ => {
                println!("Failed2");
                assert_eq!(false, true);
            }
        },
        None => {
            assert_eq!(true, true)
        }
    }
}

#[test]
fn test_basic() {
    // solana_logger::setup_with_default("solana_program_runtime=debug");
    // solana_logger::setup_with_default("solana_runtime::message=debug");

    let mut testvalgen = TestValidatorGenesis::default();

    let initializer = add_account(&mut testvalgen);
    let alice = add_account(&mut testvalgen);
    let bob = add_account(&mut testvalgen);
    let diana = add_account(&mut testvalgen);

    let (test_validator, _payer) = testvalgen
        .add_program("target/deploy/bpf_program_template", id())
        .start();
    let rpc_client = test_validator.get_rpc_client();

    let alice_party: String = String::from("Alice Party");
    let diana_party: String = String::from("Diana Party");

    initialize(&rpc_client, &initializer,0);

    create_party(&rpc_client, &initializer, &alice, &alice_party,0);

    create_party(&rpc_client, &initializer, &diana, &diana_party,0);

    create_voter(&rpc_client, &initializer.pubkey(), &bob,0);

    vote_positive(&rpc_client, &initializer.pubkey(), &bob, &alice_party,0);

    vote_positive(&rpc_client, &initializer.pubkey(), &bob, &alice_party,19);

    vote_negative(&rpc_client, &initializer.pubkey(), &bob, &alice_party,23);

    vote_positive(&rpc_client, &initializer.pubkey(), &bob, &diana_party,0);

    vote_negative(&rpc_client, &initializer.pubkey(), &bob, &alice_party,0);

    vote_negative(&rpc_client, &initializer.pubkey(), &bob, &alice_party,22);
}

#[test]
fn happy_path1() {
    // solana_logger::setup_with_default("solana_program_runtime=debug");
    // solana_logger::setup_with_default("solana_runtime::message=debug");

    let mut testvalgen = TestValidatorGenesis::default();

    let initializer = add_account(&mut testvalgen);
    let alice = add_account(&mut testvalgen);
    let bob = add_account(&mut testvalgen);
    let diana = add_account(&mut testvalgen);
    let michael = add_account(&mut testvalgen);

    let alice_party: String = String::from("Alice Party");
    let diana_party: String = String::from("Diana Party");
    let michael_party: String = String::from("Michael Party");

    let (test_validator, _payer) = testvalgen
        .add_program("target/deploy/bpf_program_template", id())
        .start();

    let rpc_client = test_validator.get_rpc_client();

    initialize(&rpc_client, &initializer,0);

    create_party(&rpc_client, &initializer, &alice, &alice_party,0);

    create_party(&rpc_client, &initializer, &michael, &michael_party,0);

    create_party(&rpc_client, &initializer, &diana, &diana_party,0);

    create_voter(&rpc_client, &initializer.pubkey(), &bob,0);

    vote_positive(&rpc_client, &initializer.pubkey(), &bob, &alice_party,0);

    vote_positive(&rpc_client, &initializer.pubkey(), &bob, &michael_party,0);

    vote_negative(&rpc_client, &initializer.pubkey(), &bob, &diana_party,0);

    vote_positive(&rpc_client, &initializer.pubkey(), &bob, &diana_party,22);

    vote_negative(&rpc_client, &initializer.pubkey(), &bob, &alice_party,22);
}

#[test]
fn happy_path2() {
    // solana_logger::setup_with_default("solana_program_runtime=debug");
    // solana_logger::setup_with_default("solana_runtime::message=debug");

    let mut testvalgen = TestValidatorGenesis::default();

    let initializer = add_account(&mut testvalgen);
    let alice = add_account(&mut testvalgen);
    let bob = add_account(&mut testvalgen);

    let alice_party: String = String::from("Alice Party");

    let (test_validator, _payer) = testvalgen
        .add_program("target/deploy/bpf_program_template", id())
        .start();

    let rpc_client = test_validator.get_rpc_client();

    initialize(&rpc_client, &initializer,0);

    create_party(&rpc_client, &initializer, &alice, &alice_party,0);

    create_voter(&rpc_client, &initializer.pubkey(), &bob,0);

    vote_negative(&rpc_client, &initializer.pubkey(), &bob, &alice_party,23);
}

#[test]
fn happy_path3() {
    //solana_logger::setup_with_default("solana_program_runtime=debug");
    //solana_logger::setup_with_default("solana_runtime::message=debug");

    let mut testvalgen = TestValidatorGenesis::default();

    let initializer = add_account(&mut testvalgen);
    let alice = add_account(&mut testvalgen);

    let (test_validator, _payer) = testvalgen
        .add_program("target/deploy/bpf_program_template", id())
        .start();

    let rpc_client = test_validator.get_rpc_client();

    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let (pda_owner, bump_owner) =
        Pubkey::find_program_address(&[b"voting_owner", initializer.pubkey().as_ref()], &id());

    let (pda_state, bump_state) =
        Pubkey::find_program_address(&[b"voting_state", pda_owner.as_ref()], &id());

    let instruction_data = vec![0u8, bump_owner, bump_state];

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(initializer.pubkey(), false), // initializer
                AccountMeta::new(pda_owner, false),            // voting owner
                AccountMeta::new(pda_state, false),            // voting state
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
                AccountMeta::new(alice.pubkey(), true),
            ],
            data: instruction_data,
        }],
        Some(&alice.pubkey()),
    );
    transaction.sign(&[&alice], blockhash);

    // no instruction err
    assert_matches!(rpc_client.send_and_confirm_transaction(&transaction),Err(_));

}

#[test]
fn happy_path4() {
    let mut testvalgen = TestValidatorGenesis::default();

    let initializer = add_account(&mut testvalgen);
    let voter = add_account(&mut testvalgen);

    let (test_validator, _payer) = testvalgen
        .add_program("target/deploy/bpf_program_template", id())
        .start();

    let rpc_client = test_validator.get_rpc_client();

    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let (pda_owner, bump_owner) =
        Pubkey::find_program_address(&[b"voting_owner", initializer.pubkey().as_ref()], &id());

    let (pda_state, bump_state) =
        Pubkey::find_program_address(&[b"voting_state", pda_owner.as_ref()], &id());

    let (pda_voter, bump_voter) = Pubkey::find_program_address(
        &[b"new_voter", voter.pubkey().as_ref(), pda_state.as_ref()],
        &id(),
    );

    let instruction_data = vec![2u8, bump_owner, bump_state, bump_voter];

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(voter.pubkey(), true), // persone that wants to be voter
                AccountMeta::new_readonly(initializer.pubkey(), false), // owner
                AccountMeta::new_readonly(pda_owner, false), // voting owner
                AccountMeta::new_readonly(pda_state, false), // voting state
                AccountMeta::new(pda_voter, false),     // voter
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
            data: instruction_data,
        }],
        Some(&voter.pubkey()),
    );
    transaction.sign(&[&voter], blockhash);
    compare_error(rpc_client.send_and_confirm_transaction(&transaction), 7);
}
