//! Account state definitions for the E9th Token Program

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey,
};

/// Token configuration account
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct TokenConfig {
    /// Multisig or super admin
    pub owner: Pubkey,
    /// Pause, blacklist, airdrop operations
    pub operator: Pubkey,
    /// Holds reward pool, etc.
    pub treasury: Pubkey,
    /// Whether token transfers are paused
    pub is_paused: bool,
    /// Burn rate in basis points (e.g. 100 = 1%)
    pub burn_rate_basis_points: u16,
    /// Token mint
    pub mint: Pubkey,
    /// Total supply
    pub total_supply: u64,
    /// Staking enabled
    pub staking_enabled: bool,
    /// Reward rate per epoch (in basis points)
    pub reward_rate: u16,
    /// Minimum stake period (in epochs)
    pub min_stake_period: u64,
    /// Maximum stake period (in epochs)
    pub max_stake_period: u64,
    /// Total staked amount
    pub total_staked: u64,
    /// Bump seed for PDA
    pub bump: u8,
}

/// Blacklist account
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Blacklist {
    /// List of blacklisted accounts
    pub accounts: Vec<Pubkey>,
    /// Bump seed for PDA
    pub bump: u8,
}

/// Enhanced stake entry
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct StakeEntry {
    /// Staker's public key
    pub staker: Pubkey,
    /// Amount staked
    pub amount: u64,
    /// Last reward timestamp
    pub last_reward_timestamp: u64,
    /// Stake start epoch
    pub start_epoch: u64,
    /// Stake period (in epochs)
    pub period: u64,
    /// Rewards claimed
    pub rewards_claimed: u64,
    /// Lock time (for early unstake penalties)
    pub lock_time: u64,
    /// Bump seed for PDA
    pub bump: u8,
}

/// Program state account (legacy - keeping for compatibility)
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct ProgramState {
    /// Admin authority
    pub admin: Pubkey,
    /// Token mint
    pub mint: Pubkey,
    /// Total supply
    pub total_supply: u64,
    /// Staking enabled
    pub staking_enabled: bool,
    /// Reward rate per epoch (in basis points)
    pub reward_rate: u16,
    /// Minimum stake period (in epochs)
    pub min_stake_period: u64,
    /// Maximum stake period (in epochs)
    pub max_stake_period: u64,
    /// Total staked amount
    pub total_staked: u64,
    /// Bump seed for PDA
    pub bump: u8,
}

impl TokenConfig {
    pub const LEN: usize = 32 + 32 + 32 + 1 + 2 + 32 + 8 + 1 + 2 + 8 + 8 + 8 + 1; // 201 bytes

    pub fn new(
        owner: Pubkey,
        operator: Pubkey,
        treasury: Pubkey,
        mint: Pubkey,
        burn_rate_basis_points: u16,
        bump: u8,
    ) -> Self {
        Self {
            owner,
            operator,
            treasury,
            is_paused: false,
            burn_rate_basis_points,
            mint,
            total_supply: 0,
            staking_enabled: true,
            reward_rate: 100, // 1% per epoch
            min_stake_period: 1,
            max_stake_period: 365,
            total_staked: 0,
            bump,
        }
    }
}

impl Blacklist {
    pub const LEN: usize = 4 + (32 * 100) + 1; // 4 bytes for Vec length + 100 accounts max + bump

    pub fn new(bump: u8) -> Self {
        Self {
            accounts: Vec::new(),
            bump,
        }
    }

    pub fn is_blacklisted(&self, account: &Pubkey) -> bool {
        self.accounts.contains(account)
    }

    pub fn add_account(&mut self, account: Pubkey) {
        if !self.accounts.contains(&account) {
            self.accounts.push(account);
        }
    }

    pub fn remove_account(&mut self, account: &Pubkey) {
        self.accounts.retain(|a| a != account);
    }
}


impl ProgramState {
    pub const LEN: usize = 32 + 32 + 8 + 1 + 2 + 8 + 8 + 8 + 1; // 99 bytes

    pub fn new(admin: Pubkey, mint: Pubkey, bump: u8) -> Self {
        Self {
            admin,
            mint,
            total_supply: 0,
            staking_enabled: true,
            reward_rate: 100, // 1% per epoch
            min_stake_period: 1,
            max_stake_period: 365,
            total_staked: 0,
            bump,
        }
    }
}


/// Stake account state (legacy - keeping for compatibility)
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct StakeAccount {
    /// Owner of the stake
    pub owner: Pubkey,
    /// Amount staked
    pub amount: u64,
    /// Stake start epoch
    pub start_epoch: u64,
    /// Stake period (in epochs)
    pub period: u64,
    /// Rewards claimed
    pub rewards_claimed: u64,
    /// Bump seed for PDA
    pub bump: u8,
}

impl StakeEntry {
    pub const LEN: usize = 32 + 8 + 8 + 8 + 8 + 8 + 8 + 1; // 89 bytes

    pub fn new(
        staker: Pubkey,
        amount: u64,
        period: u64,
        start_epoch: u64,
        lock_time: u64,
        bump: u8,
    ) -> Self {
        Self {
            staker,
            amount,
            last_reward_timestamp: 0,
            start_epoch,
            period,
            rewards_claimed: 0,
            lock_time,
            bump,
        }
    }

    /// Calculate pending rewards based on time
    pub fn calculate_rewards(&self, current_timestamp: u64, reward_rate: u16) -> u64 {
        if current_timestamp <= self.last_reward_timestamp {
            return 0;
        }

        let time_elapsed = current_timestamp - self.last_reward_timestamp;
        let reward_per_second = (self.amount as u128 * reward_rate as u128) / (10000 * 365 * 24 * 60 * 60); // per second
        let total_rewards = reward_per_second * time_elapsed as u128;
        
        total_rewards.saturating_sub(self.rewards_claimed as u128) as u64
    }

    /// Check if stake is mature
    pub fn is_mature(&self, current_epoch: u64) -> bool {
        current_epoch >= self.start_epoch + self.period
    }

    /// Check if lock time has passed
    pub fn is_unlocked(&self, current_timestamp: u64) -> bool {
        current_timestamp >= self.lock_time
    }
}

impl StakeAccount {
    pub const LEN: usize = 32 + 8 + 8 + 8 + 8 + 1; // 65 bytes

    pub fn new(owner: Pubkey, amount: u64, period: u64, start_epoch: u64, bump: u8) -> Self {
        Self {
            owner,
            amount,
            start_epoch,
            period,
            rewards_claimed: 0,
            bump,
        }
    }

    /// Calculate pending rewards
    pub fn calculate_rewards(&self, current_epoch: u64, reward_rate: u16) -> u64 {
        if current_epoch <= self.start_epoch {
            return 0;
        }

        let elapsed_epochs = current_epoch - self.start_epoch;
        if elapsed_epochs < self.period {
            return 0; // Stake not mature yet
        }

        // Calculate rewards based on the staking period and reward rate
        let reward_per_epoch = (self.amount as u128 * reward_rate as u128) / 10000;
        let total_rewards = reward_per_epoch * self.period as u128;
        
        total_rewards.saturating_sub(self.rewards_claimed as u128) as u64
    }

    /// Check if stake is mature
    pub fn is_mature(&self, current_epoch: u64) -> bool {
        current_epoch >= self.start_epoch + self.period
    }
}

/// Helper function to deserialize account data
pub fn deserialize_account_data<T: BorshDeserialize>(
    account_info: &AccountInfo,
) -> Result<T, ProgramError> {
    let data = account_info.try_borrow_data()?;
    T::try_from_slice(&data).map_err(|_| ProgramError::InvalidAccountData)
}

/// Helper function to serialize account data
pub fn serialize_account_data<T: BorshSerialize>(
    account_info: &AccountInfo,
    data: &T,
) -> Result<(), ProgramError> {
    let mut account_data = account_info.try_borrow_mut_data()?;
    let serialized = data.try_to_vec().map_err(|_| ProgramError::InvalidAccountData)?;
    account_data[..serialized.len()].copy_from_slice(&serialized);
    Ok(())
}
