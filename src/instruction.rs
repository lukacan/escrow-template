
pub mod instruction {
    use borsh::{BorshDeserialize, BorshSerialize};

    #[derive(Debug,BorshDeserialize,BorshSerialize)]
    pub struct CreateParty {
        pub name: String,
    }

    #[derive(Debug,BorshSerialize,BorshDeserialize)]
    pub struct Vote{
        pub party_name:String
    }
}