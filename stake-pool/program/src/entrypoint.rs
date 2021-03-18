//! Program entrypoint

/// 
#![cfg(all(target_arch = "bpf", not(feature = "no-entrypoint")))]
// Ask : Understand above statement in context of Solana 


use crate::{error::StakePoolError, processor::Processor};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    program_error::PrintProgramError, pubkey::Pubkey,
};

// Ask : what does the ! here mean : syntactica?
entrypoint!(process_instruction);
fn process_instruction(
    // David : What would be the typical input by the user here?
    // Similar to Solidity : a (contract) address of where this will be deployed? 
    // What's the ID being referred to in lib.rs at Line 20 
    // solana_program::declare_id!("poo1B9L9nR3CrcaziKVYVpRX6A9Y1LAXYasjjfCbApj");


    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<StakePoolError>();
        return Err(error);
    }
    Ok(())
}
