# Janecek Method

## Rules:
- Only owner can register party into the vote
- Every voter has 2 positive and 1 negative votes
- Negative vote can be given only after 2 positive
- One party can receive only 1 positive vote from the voter, no restriction for negative vote (can spent negative vote to party you voted positive)
- Voting ends 7 days after deployment
- Everyone can see estimated time to the end (in seconds)
- Everyone can vote and see vote results live, but you have to be registered as voter

## Owner
- A voting owner is an account fully controlled by the app.

## Voting state/owner
- A voting state is an account created by the app during the start (multiple voting states can active simultaneously) and associated with the voting owner. Voters are allowed to vote for 1 week - the deadline is set in the voting state account.

## Voter
- A voter is a regular Solana account. The app makes the voter eligible for voting by creating a voter votes account associated with the given voter and the voting state.

## Party
- A party is an account created by the app with the required party name and associated with the voting state.


### Initialize
- Voting owner = PDA derived from "voting_owner" and author`s public key.
- Voting state = PDA derived from "voting_state" and address of voting owner PDA.
- Thanks to canonical property of find_program_address, author can be tied up to only one voting_owner same as to only one voting state.
- Multiple voting owners with voting states can "live" simultaneously.
- Author is responsible for paying fees.
### CreateParty
- Party = PDA derived from its name and address of voting state PDA.
- That means party name has to be uniqe in context of one voting state
- Author is responsible for paing fees.
- Author and Owner, who corresponds to this voting state, have to sign transaction - meaning owner has to approve inserting new party into context.
- Party in specified context can`t be created after voting in the context ended.
### CreateVoter
- Voter = PDA derived from "new_voter", author`s public key and address of voting state PDA.
- This ensures that author cannot create multiple voters in specified voting state.
- Author pais fees for creating voter account. 
- No need for owner approval to become new Voter, one restriction is that u have to provide context in which u want to vote, so u can`t be voter without context.
- Voter in specified context can`t be created after voting in the context ended.
### Vote
- Perforsm voting based on specified rules with provided voting state.

## Security checks
- todo 

### Environment Setup
1. Install Rust from https://rustup.rs/
2. Install Solana from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

### Build and test the program compiled for SBF
```
$ cargo build-sbf
$ cargo test-sbf
```
