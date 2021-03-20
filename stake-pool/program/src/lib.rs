#![deny(missing_docs)]

//! A program for creating pools of Solana stakes managed by a Stake-o-Matic

pub mod error;
pub mod instruction;
pub mod processor;
pub mod stake;
pub mod state;

/// Current program version
pub const PROGRAM_VERSION: u8 = 1;

// Ask : What? Fynn/David
#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

// Ask : David : What is this address refering to? 
// Ankit : Shouldn't this be dynamically set on deployment
// David : Yeah, I am confused too. 
// It's optional, but if you use it, you are asking "the blockchain" to deploy this program at this specific address. 
solana_program::declare_id!("poo1B9L9nR3CrcaziKVYVpRX6A9Y1LAXYasjjfCbApj");
