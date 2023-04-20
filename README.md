# **Janecek Method**

## 🍇***Rules***:
- <u>Only owner can register party into the vote</u>
- <u>Every voter has 2 positive and 1 negative votes</u>
- <u>Negative vote can be given only after 2 positive</u>
- <u>One party can receive only 1 positive vote from the voter, no restriction for negative vote (can spent negative vote to party you voted positive)</u>
- <u>Voting ends 7 days after deployment</u>
- <u>Everyone can see estimated time to the end (in seconds)</u>
- <u>Everyone can vote and see vote results live</u>

## 🍉***Owner***
- **Voting owner** = PDA derived from "voting_owner" and author`s public key.
## 🍊***Voting state***
- **Voting state** = PDA derived from "voting_state" and address of voting owner PDA.
- A **voting state** is an account created by the app during the start (multiple voting states can active simultaneously) and associated with the **voting owner**. Voters are allowed to vote for 1 week - the deadline is set in the voting state account.
- Thanks to **canonical property of find_program_address**, author can be tied up to only one voting_owner same as to only one voting state.
- Multiple voting owners with voting states can "live" simultaneously.
- **Author** is responsible for paying **fees**.

## 🍋***Voter***
- **Voter** = PDA derived from "new_voter", author`s public key and address of voting state PDA.
- This ensures that author cannot create multiple voters in specified voting state.
- **Author pais fees** for creating voter account.
- No need for owner approval to become new Voter, one restriction is that you have to provide context in which you want to vote, so you can`t be voter without context.
- Voter in specified context can`t be created after voting ended.
## 🍍***Party***
- **Party** = PDA derived from its name and address of voting state PDA.
- That means party name has to be uniqe in context of one voting state
- **Author** is responsible for paing **fees**.
- **Author and Owner, who corresponds to this voting state, have to sign transaction - meaning owner has to approve inserting new party into context.**
- Party in specified context can`t be created after voting in the context ended.

✅
### 🍎***Initialize***
- **Signer Check**
- **PDA Ownership**
- **System Program ID**
- **Writable check**
- **PDAs and Bumps check**
- **Rent exempt - State/Owner**
- **Reinitialization check**
- **Addition overflow check**
- **Read only accounts**
- **IX data length check**
### 🍓***CreateParty***
- **Signers Check**
- **PDA Ownershop**
- **System Program ID**
- **Writable check**
- **PDAs and Bumps check**
- **Rent exempt - State/Owner/Party**
- **Reinitialization check**
- **Not Yet initialized check**
- **Owner/Voting Owner/Voting State corresponds**
- **Voting Ended check**
- **Bumps inside accounts check**
- **Read only accounts**
- **IX data length check**
### 🥝***CreateVoter***
- **Signer Check**
- **PDA Ownership**
- **System Program ID**
- **Writable check**
- **PDAs and Bumps check**
- **Rent exempt - State/Owner/Voter**
- **Reinitialization check**
- **Not Yet initialized check**
- **Owner/Voting Owner/Voting State corresponds**
- **Voting Ended check**
- **Bumps inside accounts check**
- **Read only accounts**
- **IX data length check**
### 🍒***Vote***
- **Signer Check**
- **PDA Ownership**
- **Writable check**
- **PDAs and Bumps check**
- **Not Yet initialized check**
- **Author/Voter corresponds**
- **Party/Voting State corresponds**
- **Voter/Voting State corresponds**
- **Owner/Voting Owner/Voting State corresponds**
- **Voting Ended check**
- **No both positive votes to single Party**
- **Addition/Subtraction overflow check**
- **Desirable states of number of votes**
- **Negative vote after both positive spent**
- **Bumps inside accounts check**
- **Read only accounts**
- **Rent exempt - State/Owner/Party/Voter**
- **IX data length check**


## 🥥***Security checks***
- **Signer Checks** to verify that specific accounts have signed a transaction.
- Use **Owner Checks** to verify that accounts are owned by the expected program.
- Use **Data Validation** checks to verify that account data matches an expected value, Owner/Voting Owner/Voting State/Party/Voter/Author corresponds to each other.
- Use an **Account Discriminator** or **Initialization Flag** to check whether an account has already been initialized to prevent an account from being reinitialized and overriding existing account data.
- When an instruction requires two mutable accounts of the same type, an attacker can pass in the same account twice, causing the account to be mutated in unintended ways (not using instructions this way, but discrimantor considered).
- Use **discriminators** to distinguish between different account types.
- Perform **program checks in native programs** by simply comparing the public key of the passed-in program to the progam you expected.
- Using **find_program_address** ensures that the highest valid bump, or canonical bump, is used for the derivation, thus creating a deterministic way to find an address given specific seeds.
- Using the same PDA for multiple authority domains opens your program up to the possibility of users accessing data and funds that don't belong to them
- Check writable accounts.
- Check read-only accounts.
- Check rent exempt.
- Check bumps vs saved bumps.
- Double check correct program ID.
- 🟠Test IT!




## 🍌***Environment Setup***
1. Install Rust from https://rustup.rs/
2. Install Solana from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

## 🥩***Build and test the program compiled for SBF***
```
$ cargo build-sbf
$ cargo test-sbf
```
