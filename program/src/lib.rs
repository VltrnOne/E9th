//! E9th Token Program
//! A Solana program for creating and managing custom tokens with staking functionality

pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod admin;
pub mod stake;

// Re-export key types for easier access
pub use error::E9thTokenError;
pub use instruction::{E9thInstruction, E9thTokenInstruction};
pub use state::*;

// Program ID - this will be set when the program is deployed
solana_program::declare_id!("E9thToken11111111111111111111111111111111111");

#[cfg(not(feature = "no-entrypoint"))]
pub use entrypoint::process_instruction;
