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

### ***Questions***
- ❗❓ **Is there a way to nicely check what error was returned from rpc client during transaction ??** ❌
- ❓ **not using string in instruction data, but bytearray good/bad = good ✅**
- ❓ **deserialization on enum - what if i send instraction data too long, have test case for this = yep, check if all data deserialized✅**
- ❓ **readability checks = no need to check this✅**
- ❓ **system program owner check during initi/c_party/c_voter = no need to check this CPI, not possible with this✅**
- ❓ **error order, if voting state is not initalized and user wants to create party, owner check triggers first (account does not exists, so owner is system program), after that account initialized check is performed, but this behaviour can confuse users = not so important, provide good documentation for this✅**

### 🍎***Initialize***
- 🔴✅**Signer check**
- 🔴✅**Provided PDA == Derived PDA**
- 🔴✅**Correct System Program ID**
- 🔴✅**Ownership check**
- 🔴✅**Rent Exempt**
- 🔴✅**Already Initialized**
### 🍓***CreateParty***
- 🔴✅**Signers check**
- 🔴✅**Provided PDA == Derived PDA**
- 🔴✅**Correct System Program ID**
- 🔴✅**Ownership check**
- 🔴✅**Rent Exempt**
- 🔴✅**Already Initialized**
- 🔵✅**Account Uninitialized**
- 🔵✅**Bumps check**
- 🔵✅**Voting Ended**
- 🔵✅**Voting Owner/ Voting State/ Initializer Check**
- 🟤✅**String no longer than 32 bytes check**
### 🥝***CreateVoter***
- 🔴✅**Signer check**
- 🔴✅**Provided PDA == Derived PDA**
- 🔴✅**Correct System Program ID**
- 🔴✅**Ownership check**
- 🔴✅**Rent Exempt**
- 🔴✅**Already Initialized**
- 🔵✅**Account Uninitialized**
- 🔵✅**Bumps check**
- 🔵✅**Voting Ended**
- 🔵✅**Voting Owner/ Voting State/ Initializer Check**
### 🍒***Vote***
- 🔴✅**Signer check**
- 🔴✅**Provided PDA == Derived PDA**
- 🔴✅**Ownership check**
- 🔴✅**Rent Exempt**
- 🔵✅**Account Uninitialized**
- 🔵✅**Bumps check**
- 🔵✅**Voting Ended**
- 🔵✅**Voting Owner/ Voting State/ Initializer Check**
- 🟣✅**Party/ Voting State Check**
- 🟣✅**Voter/ Voting State Check**
- 🟣✅**Author/ Voter Check**
- 🟣✅**No both positive votes to single Party**
- 🟣✅**Negative vote after both positive spent**
- 🟤✅**String no longer than 32 bytes check**




## 🥥***Security checks***
- ✅**Signer Checks** to verify that specific accounts have signed a transaction.
- ✅Use **Owner Checks** to verify that accounts are owned by the expected program.
- ✅Use **Data Validation** checks to verify that account data matches an expected value, Owner/Voting Owner/Voting State/Party/Voter/Author corresponds to each other.
- ✅Use an **Account Discriminator** or **Initialization Flag** to check whether an account has already been initialized to prevent an account from being reinitialized and overriding existing account data.
- ✅When an instruction requires two mutable accounts of the same type, an attacker can pass in the same account twice, causing the account to be mutated in unintended ways (not using instructions this way, but discrimantor considered).
- ✅Use **discriminators** to distinguish between different account types.
- ✅Perform **program checks in native programs** by simply comparing the public key of the passed-in program to the progam you expected.
- ✅Using **find_program_address** ensures that the highest valid bump, or canonical bump, is used for the derivation, thus creating a deterministic way to find an address given specific seeds.
- ✅Using the same PDA for multiple authority domains opens your program up to the possibility of users accessing data and funds that don't belong to them
- ✅Check rent exempt.
- ✅Check bumps vs saved bumps.
- ✅Double check correct program ID.
- 🟠Test IT!




## 🍌***Environment Setup***
1. Install Rust from https://rustup.rs/
2. Install Solana from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

## 🥩***Build and test the program compiled for SBF***
```
$ cargo build-sbf
$ cargo test-sbf
```
## 🥓***Test specific instruction (create_party)***
```
$ cargo test-sbf --test create_party
```
## 🍗***Test specific test in specific instruction***
```
$ cargo test-sbf --test create_party basic1
```
