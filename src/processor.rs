use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};


use crate::{error::JanecekError, 
    instruction::JanecekMethodInstructions};
use crate::identifier::ID;



pub struct Processor;
impl Processor {
    pub fn entry(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        Self::try_entry(program_id, accounts, instruction_data)
        .map_err(|e| {
            //e.log();
            e.into()
        })
    }
    fn try_entry(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
    ) -> ProgramResult {
        
        // check if program ID is correct
        if *program_id != ID {
            return Err(JanecekError::ProgramIDMismatch.into());
        }

        // check if data contains at least 1 byte, so function can be decoded
        if data.len() < 1 {
            return Err(JanecekError::MissmatchInstruction.into());
        }
        Self::dispatch(program_id, accounts, data)
    }

    fn dispatch(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
    ) -> ProgramResult {
        let (tag, rest) = data.split_first().ok_or(JanecekError::InvalidInstruction)?;
        let mut ix_data: &[u8] = data;
        match tag {
            0 => {
                JanecekMethodInstructions::create_party()
            }
            1 => {
                JanecekMethodInstructions::create_voter()
            }
            2 => {
                JanecekMethodInstructions::vote_positive()
            }
            3 => {
                JanecekMethodInstructions::vote_negative()
            }
            _ => Err(JanecekError::InstructionFallbackNotFound.into()),
        }
    }
}
















//     fn process_init_escrow(
//         accounts: &[AccountInfo],
//         amount: u64,
//         program_id: &Pubkey,
//     ) -> ProgramResult {
//         let account_info_iter = &mut accounts.iter();
//         let initializer = next_account_info(account_info_iter)?;

//         if !initializer.is_signer {
//             return Err(ProgramError::MissingRequiredSignature);
//         }

//         let temp_token_account = next_account_info(account_info_iter)?;

//         let token_to_receive_account = next_account_info(account_info_iter)?;
//         if *token_to_receive_account.owner != spl_token::id() {
//             return Err(ProgramError::IncorrectProgramId);
//         }

//         let escrow_account = next_account_info(account_info_iter)?;
//         let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

//         if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
//             return Err(EscrowError::NotRentExempt.into());
//         }

//         let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.try_borrow_data()?)?;
//         if escrow_info.is_initialized() {
//             return Err(ProgramError::AccountAlreadyInitialized);
//         }

//         escrow_info.is_initialized = true;
//         escrow_info.initializer_pubkey = *initializer.key;
//         escrow_info.temp_token_account_pubkey = *temp_token_account.key;
//         escrow_info.initializer_token_to_receive_account_pubkey = *token_to_receive_account.key;
//         escrow_info.expected_amount = amount;

//         Escrow::pack(escrow_info, &mut escrow_account.try_borrow_mut_data()?)?;
//         let (pda, _bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);

//         let token_program = next_account_info(account_info_iter)?;
//         let owner_change_ix = spl_token::instruction::set_authority(
//             token_program.key,
//             temp_token_account.key,
//             Some(&pda),
//             spl_token::instruction::AuthorityType::AccountOwner,
//             initializer.key,
//             &[&initializer.key],
//         )?;

//         msg!("Calling the token program to transfer token account ownership...");
//         invoke(
//             &owner_change_ix,
//             &[
//                 temp_token_account.clone(),
//                 initializer.clone(),
//                 token_program.clone(),
//             ],
//         )?;

//         Ok(())
//     }
// }
