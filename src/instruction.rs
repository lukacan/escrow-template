use std::convert::TryInto;
use solana_program::program_error::ProgramError;
use crate::error::JanecekError;


pub enum JanecekMethodInstruction{
    CreateParty{
        name: String
    },
    CreateVoter{

    },
    VotePositive{
        party_name: String
    },
    VoteNegative{
        party_name: String
    },
}

impl JanecekMethodInstruction{
    pub fn unpack(input: &[u8]) -> Result<Self,ProgramError>{
        let (tag, rest) = input.split_first().ok_or(JanecekError::InvalidInstruction)?;
    
        
        Ok(match tag {
            0 => Self::CreateParty {
                name: Self::unpack_name(rest)?,
            },
            1 => Self::CreateVoter {  },
            2 => Self::VoteNegative {
                party_name: Self::unpack_name(rest)?,  
            },
            3 => Self::VotePositive {
                party_name: Self::unpack_name(rest)?,  
            },
            _ => return Err(JanecekError::InvalidInstruction.into()),
        })
    }
    fn unpack_name(input: &[u8]) -> Result<String, ProgramError> {
        let string_bytes = input.get(..input.len()).ok_or(JanecekError::InvalidInstruction)?;
        let name = std::str::from_utf8(string_bytes)
            .map_err(|_| JanecekError::InvalidInstruction)?
            .to_owned();
        Ok(name)
    }
}









// pub mod JanecekMethodInstructions{
//     use solana_program::entrypoint::ProgramResult;

//     pub fn create_party() -> ProgramResult{
//         Ok(())
//     }
//     pub fn create_voter() -> ProgramResult{
//         Ok(())
//     }
//     pub fn vote_positive() -> ProgramResult{
//         Ok(())
//     }
//     pub fn vote_negative() -> ProgramResult{
//         Ok(())
//     }
// }