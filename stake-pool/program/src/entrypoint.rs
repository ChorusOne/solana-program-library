//! Program entrypoint

/// 
#![cfg(all(target_arch = "bpf", not(feature = "no-entrypoint")))]
// Ask : Understand above statement in context of Solana 
// !Ignore it

use crate::{error::StakePoolError, processor::Processor};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    program_error::PrintProgramError, pubkey::Pubkey,
};



// Ask : what does the ! here mean : syntactica?
entrypoint!(process_instruction);
// This is where my contract starts running
// ankit sends a transaction to each blockchain -> validator runs this instruction 
fn process_instruction(
    // David : What would be the typical input by the user here?
    // Similar to Solidity : a (contract) address of where this will be deployed? 
    // What's the ID being referred to in lib.rs at Line 20 
    // solana_program::declare_id!("poo1B9L9nR3CrcaziKVYVpRX6A9Y1LAXYasjjfCbApj");

    // When you deploy this Stake-Pool contracts : you will get an address
    // That address will be passed here
    
    // The canonical Stake Pool program has program ID : poo1B9L9nR3CrcaziKVYVpRX6A9Y1LAXYasjjfCbApj
    // Other people can also deploy this => new program IDs => which would be here 

    // 1 Stake Pool Program => Stake Pools (With Each Having an Owner) => Across Validator 
    // David : 1 Stake Pool Program = 1 Stake Pool
    // Ankit : I have my doubts, let's come back to this! 
    program_id: &Pubkey,
    // At this point, Solana loads info of all accounts mentioned here into data structures of the form account:info
    accounts: &[AccountInfo],
    // A reference/pointer to a u8 array 
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<StakePoolError>();
        return Err(error);
    }
    Ok(())
}
