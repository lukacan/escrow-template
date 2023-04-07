#![cfg(feature = "test-bpf")]


pub static ID: solana_program::pubkey::Pubkey = solana_program::pubkey::Pubkey::new_from_array([
    219u8,
    176u8,
    239u8,
    163u8,
    143u8,
    88u8,
    163u8,
    57u8,
    53u8,
    33u8,
    238u8,
    94u8,
    174u8,
    25u8,
    76u8,
    161u8,
    165u8,
    165u8,
    87u8,
    163u8,
    23u8,
    199u8,
    180u8,
    248u8,
    26u8,
    103u8,
    217u8,
    114u8,
    113u8,
    69u8,
    43u8,
    22u8,
]);


use {
    assert_matches::*,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_sdk::{signature::Signer, transaction::Transaction},
    solana_validator::test_validator::*,
};

#[test]
#[ignore = "reason"]
fn test_validator_transaction() {
    solana_logger::setup_with_default("solana_program_runtime=debug");
    //let program_id = Pubkey::new_unique();

    let program_id = ID;

    
    let (test_validator, payer) = TestValidatorGenesis::default()
        .add_program("bpf_program_template", program_id)
        .start();
    let rpc_client = test_validator.get_rpc_client();

    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let (pda,bump) = Pubkey::find_program_address(
        &["Hello".as_bytes()], 
        &program_id,);


    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(pda,false)],
            data: vec![0 ,5, 0, 0, 0, 104, 101, 108, 108, 111,],
        }],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], blockhash);

    assert_matches!(rpc_client.send_and_confirm_transaction(&transaction), Ok(_));
}
