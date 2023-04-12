
pub mod instruction {
    use borsh::{BorshDeserialize, BorshSerialize};

    #[derive(Debug,BorshDeserialize,BorshSerialize)]
    pub struct Initialize{
        pub bump_owner:u8,
        pub bump_state:u8,
    }
    #[derive(Debug,BorshDeserialize,BorshSerialize)]
    pub struct CreateParty {
        pub bump_owner:u8,
        pub bump_state:u8,
        pub bump_party:u8,
        pub name: String,
        
    }
    #[derive(Debug,BorshDeserialize,BorshSerialize)]
    pub struct CreateVoter{
        pub bump:u8,
    }
    #[derive(Debug,BorshSerialize,BorshDeserialize)]
    pub struct Vote{
        pub bump:u8,

    }
}