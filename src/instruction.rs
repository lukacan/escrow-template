
pub mod instruction {
    use borsh::{BorshDeserialize, BorshSerialize};

    use crate::state::state::NAME_LENGTH;

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
        pub bump_owner:u8,
        pub bump_state:u8,
        pub bump_voter:u8,
    }
    #[derive(Debug,BorshSerialize,BorshDeserialize)]
    pub struct Vote{
        pub bump_owner:u8,
        pub bump_state:u8,
        pub bump_voter:u8,
        pub bump_party:u8,
        pub name: String,
    }
    impl Initialize{
        pub const LEN:usize = 1+1;
    }
    impl CreateParty{
        pub const LEN:usize = 1 + 1 + 1 + 4 + 4*NAME_LENGTH;
    }
    impl CreateVoter{
        pub const LEN:usize = 1 + 1 + 1;
    }
    impl Vote{
        pub const LEN:usize = 1 + 1 + 1 + 1 + 4 + 4*NAME_LENGTH;
    }
}