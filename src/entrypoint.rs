use solana_program::declare_id;
use solana_program::{
    account_info::AccountInfo,
    entrypoint, 
    entrypoint::ProgramResult, 
    pubkey::Pubkey,
};

declare_id!("Fnambs3f1XXoMmAVc94bf8t6JDAxmVkXz85XU4v2edph");

use crate::processor::Processor;

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::entry(program_id, accounts, instruction_data)
}

// #[cfg(test)]
// mod test {
//     use {
//         super::*,
//         assert_matches::*,
//         solana_program::instruction::{AccountMeta, Instruction},
//         solana_program_test::*,
//         solana_sdk::{signature::Signer, transaction::Transaction},
//     };

//     #[tokio::test]
//     #[ignore = "reason"]
//     async fn test_transaction() {
//         // let program_id = Pubkey::new_unique();
//         let program_id = identifier::ID;


//         let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
//             "bpf_program_template",
//             program_id,
//             processor!(process_instruction),
//         )
//         .start()
//         .await;

//         let mut transaction = Transaction::new_with_payer(
//             &[Instruction {
//                 program_id,
//                 accounts: vec![AccountMeta::new(payer.pubkey(), false)],
//                 data: vec![0 ,5, 0, 0, 0, 104, 101, 108, 108, 111,],
//             }],
//             Some(&payer.pubkey()),
//         );
//         transaction.sign(&[&payer], recent_blockhash);

//         assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
//     }
// }