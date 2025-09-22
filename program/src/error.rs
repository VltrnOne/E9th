//! Error types for the E9th Token Program

use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum E9thTokenError {
    #[error("Invalid instruction")]
    InvalidInstruction,

    #[error("Invalid account owner")]
    InvalidAccountOwner,

    #[error("Invalid account data")]
    InvalidAccountData,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Account not initialized")]
    AccountNotInitialized,

    #[error("Account already initialized")]
    AccountAlreadyInitialized,

    #[error("Invalid mint")]
    InvalidMint,

    #[error("Invalid token account")]
    InvalidTokenAccount,

    #[error("Invalid stake account")]
    InvalidStakeAccount,

    #[error("Stake account not found")]
    StakeAccountNotFound,

    #[error("Invalid stake period")]
    InvalidStakePeriod,

    #[error("Stake not mature")]
    StakeNotMature,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Invalid admin")]
    InvalidAdmin,

    #[error("Math overflow")]
    MathOverflow,

    #[error("Invalid amount")]
    InvalidAmount,

    #[error("Stake period too short")]
    StakePeriodTooShort,

    #[error("Stake period too long")]
    StakePeriodTooLong,

    #[error("Reward calculation failed")]
    RewardCalculationFailed,
}

impl From<E9thTokenError> for solana_program::program_error::ProgramError {
    fn from(e: E9thTokenError) -> Self {
        solana_program::program_error::ProgramError::Custom(e as u32)
    }
}
