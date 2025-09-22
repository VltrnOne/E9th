//! Staking functionality for the E9th Token Program

use crate::{
    error::E9thTokenError,
    state::{deserialize_account_data, serialize_account_data, ProgramState, StakeAccount},
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
    system_program,
};
// Token transfer functionality would be implemented here

pub struct StakeProcessor;

impl StakeProcessor {
    /// Stake tokens
    pub fn process_stake(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
        period: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let program_state_account = next_account_info(account_info_iter)?;
        let stake_account = next_account_info(account_info_iter)?;
        let user_account = next_account_info(account_info_iter)?;
        let user_token_account = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;

        // Validate program state
        let program_state: ProgramState = deserialize_account_data(program_state_account)?;
        if !program_state.staking_enabled {
            return Err(E9thTokenError::InvalidStakePeriod.into());
        }

        if !user_account.is_signer {
            return Err(E9thTokenError::Unauthorized.into());
        }

        // Validate stake period
        if period < program_state.min_stake_period || period > program_state.max_stake_period {
            return Err(E9thTokenError::InvalidStakePeriod.into());
        }

        // Validate amount
        if amount == 0 {
            return Err(E9thTokenError::InvalidAmount.into());
        }

        // Check if stake account exists
        if stake_account.data_is_empty() {
            // For this simplified example, we'll assume the stake account is pre-created
            // In a real implementation, you would create the account here
            return Err(E9thTokenError::AccountNotInitialized.into());
        } else {
            // Initialize stake account
            let current_epoch = crate::processor::get_current_epoch();
            let stake_data = StakeAccount::new(*user_account.key, amount, period, current_epoch, 0);
            serialize_account_data(stake_account, &stake_data)?;
        }

        // Transfer tokens from user to program (simplified - in practice, you'd use a vault)
        // For this example, we'll assume the tokens are already in the program's custody
        // In a real implementation, you'd transfer tokens to a program-controlled vault

        // Update program state
        let mut program_state: ProgramState = deserialize_account_data(program_state_account)?;
        program_state.total_staked = program_state.total_staked.saturating_add(amount);
        serialize_account_data(program_state_account, &program_state)?;

        msg!("Staked {} tokens for {} epochs", amount, period);
        Ok(())
    }

    /// Unstake tokens
    pub fn process_unstake(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let program_state_account = next_account_info(account_info_iter)?;
        let stake_account = next_account_info(account_info_iter)?;
        let user_account = next_account_info(account_info_iter)?;
        let user_token_account = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;

        // Validate user
        if !user_account.is_signer {
            return Err(E9thTokenError::Unauthorized.into());
        }

        // Get stake data
        let stake_data: StakeAccount = deserialize_account_data(stake_account)?;
        if stake_data.owner != *user_account.key {
            return Err(E9thTokenError::Unauthorized.into());
        }

        // Check if stake is mature
        let current_epoch = crate::processor::get_current_epoch();
        if !stake_data.is_mature(current_epoch) {
            return Err(E9thTokenError::StakeNotMature.into());
        }

        // Transfer tokens back to user (simplified - in practice, you'd transfer from vault)
        // For this example, we'll assume the tokens are minted back to the user

        // Update program state
        let mut program_state: ProgramState = deserialize_account_data(program_state_account)?;
        program_state.total_staked = program_state.total_staked.saturating_sub(stake_data.amount);
        serialize_account_data(program_state_account, &program_state)?;

        // Clear stake account
        let mut stake_data = stake_data;
        stake_data.amount = 0;
        serialize_account_data(stake_account, &stake_data)?;

        msg!("Unstaked {} tokens", stake_data.amount);
        Ok(())
    }

    /// Claim rewards
    pub fn process_claim_rewards(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let program_state_account = next_account_info(account_info_iter)?;
        let stake_account = next_account_info(account_info_iter)?;
        let user_account = next_account_info(account_info_iter)?;
        let user_token_account = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;

        // Validate user
        if !user_account.is_signer {
            return Err(E9thTokenError::Unauthorized.into());
        }

        // Get stake data
        let mut stake_data: StakeAccount = deserialize_account_data(stake_account)?;
        if stake_data.owner != *user_account.key {
            return Err(E9thTokenError::Unauthorized.into());
        }

        // Get program state
        let program_state: ProgramState = deserialize_account_data(program_state_account)?;

        // Calculate rewards
        let current_epoch = crate::processor::get_current_epoch();
        let pending_rewards = stake_data.calculate_rewards(current_epoch, program_state.reward_rate);

        if pending_rewards == 0 {
            return Err(E9thTokenError::InvalidAmount.into());
        }

        // Transfer rewards to user (simplified - in practice, you'd mint new tokens)
        // For this example, we'll assume the rewards are minted to the user

        // Update stake data
        stake_data.rewards_claimed = stake_data.rewards_claimed.saturating_add(pending_rewards);
        serialize_account_data(stake_account, &stake_data)?;

        msg!("Claimed {} reward tokens", pending_rewards);
        Ok(())
    }

    /// Get stake account PDA
    pub fn get_stake_account_pda(
        program_id: &Pubkey,
        user: &Pubkey,
        seed: &[u8],
    ) -> Result<(Pubkey, u8), ProgramError> {
        let (pda, bump) = Pubkey::find_program_address(
            &[b"stake", user.as_ref(), seed],
            program_id,
        );
        Ok((pda, bump))
    }

    /// Get program state PDA
    pub fn get_program_state_pda(
        program_id: &Pubkey,
    ) -> Result<(Pubkey, u8), ProgramError> {
        let (pda, bump) = Pubkey::find_program_address(
            &[b"program_state"],
            program_id,
        );
        Ok((pda, bump))
    }
}
