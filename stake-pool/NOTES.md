Useful resources

* https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/
* https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/


Quick Overview

- Entrypoint > Processor > Instruction > Processor > State > Processor 
  
- Entrypoint to the program 
- Calls process_instruction
- Which calls Processor::process(program_id, accounts, instruction data)

- Control flows to Processor.rs 
- Processor.rs asks Instruction.rs to decode the instruction_data 
- Using decoded data, the processor will select the processing function
- May use state.rs to "encode state into" or "decode the state" of an account passed from entrypoint




### Entrypoint.rs 

TODO : Noob Questions for David - Walking through Entrypoint.rs



### Processor.rs
- Program Logic


### Instruction.rs
- The API 
- 
- StakePoolInstruction
  - Initialise
- 
  - CreateValidatorStakeAccount
  - AddValidatorStakeAccount
  - RemoveValidatorStakeAccount

  - UpdateListBalance
  - UpdatePoolBalance
  - Deposit
  - Withdraw
  - SetStakingAuthority
  - SetOwner


### Stake.rs
- Ask : David : At the higher level, what is stake.rs?
- Is it a  program to be cross invoked by processor.rs 
- Is it an API - ~ instruction.rs?


- StakeInstruction
  - Initialise
  - Authorise
  - DelegateStake
  - Split
  - Withdraw
  - Deactivate
  - SetupLockupNOTUSED
  - Merge

Data structures for 
    - Delegation
    - Stake
    - Meta
    - StakeState
    - Authorised
    - Lockup


Functions 



### State.rs
 - Ask : David : Where and how is this data stored?
 - In one specific account owned by program? account address - how is that determined?
Listing all the structs here 

**Stake Pool **
- Version
- Owner 
- Deposit Bump Seed
- Withdraw Bump Seed
- Validator Stake List  (another struct)
- Pool Mint 
- Owner Fee Account
- Token Program ID 
- Total Stake Under Management
- Last Epoch stake total field was updated
- Fee 

- Methods 
  - Calculate Pool Deposit Amount()
  - Calculate Pool Withdraw Amount() 
  - Calculate Lamports Amount()
  - Calculate Fee in Pool Tokens that go to ownner()
  - Check Withdraw Authority() 
  - Check Deposit Authority()
  - Check Owner()
  - Is Initialised?
  - Deserialise 
  - SErialise
  - Max Validator Stake Accounts = 10000 


**ValidatorStakeList**
    - version
    - validators : ValidatorStakeInfo (another struct)

- Methods 
  - List Version
  - Contains  Validator?
  - Find Validator with Pubkey (mutable)
  - Find Validator with Pubkey (immutable)
  - IsInit()
  - Serialise/Deserialise()


**ValidatorStakeInfo **
    - Val Account 
    - balance
    - Last Update epoch 

- Methods 
  - Deserialise()
  - Seralise() 


TODO 
- Walk through processor.rs with David?


Solana Concepts to understand
- Account model (revisit)
- Clock, Rent, Sysvar 
- solana_program


Resources 
// https://docs.rs/solana-sdk/1.5.14/solana_sdk/
// https://docs.rs/solana-client/1.5.14/solana_client/
// https://docs.rs/solana-program/1.5.14/solana_program/
// https://docs.solana.com/terminology

Rust specific
* Modules
* OOP : struct, impl 



Processor Vocabulary
- Validator Address
- (Validator) Stake Account 
- Program Address
- Withdraw Authority
- Deposit Authority 
- Bump Seed

