
pub mod instruction {
    use borsh::{BorshDeserialize, BorshSerialize};

    #[derive(Debug,BorshDeserialize,BorshSerialize)]
    pub struct CreateParty {
        pub name: String,
        pub bump:u8,
    }
    #[derive(Debug,BorshDeserialize,BorshSerialize)]
    pub struct CreateVoter{
        pub bump:u8,
    }
    #[derive(Debug,BorshSerialize,BorshDeserialize)]
    pub struct Vote{
        pub party_name:String

    }
}