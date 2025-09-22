//! Main processor for the E9th Token Program

use crate::{
    admin::AdminProcessor,
    error::E9thTokenError,
    instruction::{E9thInstruction, E9thTokenInstruction},
    stake::StakeProcessor,
    state::{TokenConfig, Blacklist},
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // Try to parse as new instruction first
        if let Ok(instruction) = E9thInstruction::unpack(instruction_data) {
            return Self::process_enhanced(program_id, accounts, instruction);
        }

        // Fall back to legacy instruction parsing
        let instruction = E9thTokenInstruction::unpack(instruction_data)?;
        Self::process_legacy(program_id, accounts, instruction)
    }

    fn process_enhanced(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction: E9thInstruction,
    ) -> ProgramResult {
        match instruction {
            E9thInstruction::Initialize {
                owner,
                operator,
                treasury,
                burn_rate_basis_points,
            } => {
                msg!("Instruction: Enhanced Initialize");
                Self::process_enhanced_initialize(
                    program_id,
                    accounts,
                    owner,
                    operator,
                    treasury,
                    burn_rate_basis_points,
                )
            }
            E9thInstruction::SetPause { pause } => {
                msg!("Instruction: Set Pause");
                Self::process_set_pause(program_id, accounts, pause)
            }
            E9thInstruction::ModifyBlacklist { account, add } => {
                msg!("Instruction: Modify Blacklist");
                Self::process_modify_blacklist(program_id, accounts, account, add)
            }
            E9thInstruction::Transfer { amount } => {
                msg!("Instruction: Enhanced Transfer");
                Self::process_enhanced_transfer(program_id, accounts, amount)
            }
            E9thInstruction::Airdrop { recipients, amounts } => {
                msg!("Instruction: Airdrop");
                Self::process_airdrop(program_id, accounts, recipients, amounts)
            }
            E9thInstruction::Stake { amount } => {
                msg!("Instruction: Enhanced Stake");
                Self::process_enhanced_stake(program_id, accounts, amount)
            }
            E9thInstruction::Unstake { amount } => {
                msg!("Instruction: Enhanced Unstake");
                Self::process_enhanced_unstake(program_id, accounts, amount)
            }
            E9thInstruction::ClaimRewards => {
                msg!("Instruction: Enhanced Claim Rewards");
                Self::process_enhanced_claim_rewards(program_id, accounts)
            }
            // Handle legacy instructions
            E9thInstruction::LegacyInitialize { total_supply, reward_rate, min_stake_period, max_stake_period } => {
                msg!("Instruction: Legacy Initialize");
                AdminProcessor::process_initialize(
                    program_id,
                    accounts,
                    total_supply,
                    reward_rate,
                    min_stake_period,
                    max_stake_period,
                )
            }
            E9thInstruction::LegacyMint { amount } => {
                msg!("Instruction: Legacy Mint");
                AdminProcessor::process_mint(program_id, accounts, amount)
            }
            E9thInstruction::LegacyBurn { amount } => {
                msg!("Instruction: Legacy Burn");
                AdminProcessor::process_burn(program_id, accounts, amount)
            }
            E9thInstruction::LegacyStake { amount, period } => {
                msg!("Instruction: Legacy Stake");
                StakeProcessor::process_stake(program_id, accounts, amount, period)
            }
            E9thInstruction::LegacyUnstake => {
                msg!("Instruction: Legacy Unstake");
                StakeProcessor::process_unstake(program_id, accounts)
            }
            E9thInstruction::LegacyClaimRewards => {
                msg!("Instruction: Legacy Claim Rewards");
                StakeProcessor::process_claim_rewards(program_id, accounts)
            }
            E9thInstruction::LegacyUpdateSettings { reward_rate, min_stake_period, max_stake_period, staking_enabled } => {
                msg!("Instruction: Legacy Update Settings");
                AdminProcessor::process_update_settings(
                    program_id,
                    accounts,
                    reward_rate,
                    min_stake_period,
                    max_stake_period,
                    staking_enabled,
                )
            }
            E9thInstruction::LegacyTransferAdmin { new_admin } => {
                msg!("Instruction: Legacy Transfer Admin");
                AdminProcessor::process_transfer_admin(program_id, accounts, new_admin)
            }
        }
    }

    fn process_legacy(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction: E9thTokenInstruction,
    ) -> ProgramResult {
        match instruction {
            E9thTokenInstruction::Initialize {
                total_supply,
                reward_rate,
                min_stake_period,
                max_stake_period,
            } => {
                msg!("Instruction: Legacy Initialize");
                AdminProcessor::process_initialize(
                    program_id,
                    accounts,
                    total_supply,
                    reward_rate,
                    min_stake_period,
                    max_stake_period,
                )
            }
            E9thTokenInstruction::Mint { amount } => {
                msg!("Instruction: Legacy Mint");
                AdminProcessor::process_mint(program_id, accounts, amount)
            }
            E9thTokenInstruction::Burn { amount } => {
                msg!("Instruction: Legacy Burn");
                AdminProcessor::process_burn(program_id, accounts, amount)
            }
            E9thTokenInstruction::Stake { amount, period } => {
                msg!("Instruction: Legacy Stake");
                StakeProcessor::process_stake(program_id, accounts, amount, period)
            }
            E9thTokenInstruction::Unstake => {
                msg!("Instruction: Legacy Unstake");
                StakeProcessor::process_unstake(program_id, accounts)
            }
            E9thTokenInstruction::ClaimRewards => {
                msg!("Instruction: Legacy Claim Rewards");
                StakeProcessor::process_claim_rewards(program_id, accounts)
            }
            E9thTokenInstruction::UpdateSettings {
                reward_rate,
                min_stake_period,
                max_stake_period,
                staking_enabled,
            } => {
                msg!("Instruction: Legacy Update Settings");
                AdminProcessor::process_update_settings(
                    program_id,
                    accounts,
                    reward_rate,
                    min_stake_period,
                    max_stake_period,
                    staking_enabled,
                )
            }
            E9thTokenInstruction::TransferAdmin { new_admin } => {
                msg!("Instruction: Legacy Transfer Admin");
                AdminProcessor::process_transfer_admin(program_id, accounts, new_admin)
            }
        }
    }
}

/// Helper function to get the program state account
pub fn get_program_state_account<'a>(
    accounts: &'a [AccountInfo<'a>],
    index: usize,
) -> Result<&'a AccountInfo<'a>, ProgramError> {
    if accounts.len() <= index {
        return Err(E9thTokenError::InvalidAccountData.into());
    }
    Ok(&accounts[index])
}

/// Helper function to get a signer account
pub fn get_signer_account<'a>(
    accounts: &'a [AccountInfo<'a>],
    index: usize,
) -> Result<&'a AccountInfo<'a>, ProgramError> {
    let account = get_program_state_account(accounts, index)?;
    if !account.is_signer {
        return Err(E9thTokenError::Unauthorized.into());
    }
    Ok(account)
}

/// Helper function to get a writable account
pub fn get_writable_account<'a>(
    accounts: &'a [AccountInfo<'a>],
    index: usize,
) -> Result<&'a AccountInfo<'a>, ProgramError> {
    let account = get_program_state_account(accounts, index)?;
    if !account.is_writable {
        return Err(E9thTokenError::InvalidAccountData.into());
    }
    Ok(account)
}

/// Helper function to validate account ownership
pub fn validate_account_owner(
    account: &AccountInfo,
    expected_owner: &Pubkey,
) -> Result<(), ProgramError> {
    if account.owner != expected_owner {
        return Err(E9thTokenError::InvalidAccountOwner.into());
    }
    Ok(())
}

/// Helper function to get current epoch (simplified - in production, get from clock sysvar)
pub fn get_current_epoch() -> u64 {
    // In a real implementation, this would get the current epoch from the clock sysvar
    // For now, we'll use a placeholder
    100
}

/// Helper function to get current timestamp (simplified - in production, get from clock sysvar)
pub fn get_current_timestamp() -> u64 {
    // In a real implementation, this would get the current timestamp from the clock sysvar
    // For now, we'll use a placeholder
    1640995200 // 2022-01-01 00:00:00 UTC
}

// Enhanced instruction implementations
impl Processor {
    /// Process enhanced initialize instruction
    fn process_enhanced_initialize(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        owner: Pubkey,
        operator: Pubkey,
        treasury: Pubkey,
        burn_rate_basis_points: u16,
    ) -> ProgramResult {
        // Implementation for enhanced initialize
        // This would create TokenConfig, Blacklist accounts, etc.
        msg!("Enhanced initialize: owner={}, operator={}, treasury={}, burn_rate={}", 
             owner, operator, treasury, burn_rate_basis_points);
        Ok(())
    }

    /// Process set pause instruction
    fn process_set_pause(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        pause: bool,
    ) -> ProgramResult {
        // Only owner can pause/unpause
        // Update TokenConfig.is_paused
        msg!("Set pause: {}", pause);
        Ok(())
    }

    /// Process modify blacklist instruction
    fn process_modify_blacklist(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        account: Pubkey,
        add: bool,
    ) -> ProgramResult {
        // Only operator or owner can modify blacklist
        // Add or remove account from Blacklist
        msg!("Modify blacklist: account={}, add={}", account, add);
        Ok(())
    }

    /// Process enhanced transfer with burn logic
    fn process_enhanced_transfer(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        // Check if paused
        // Check if sender/receiver is blacklisted
        // Calculate burn amount = amount * burn_rate / 10000
        // Transfer amount - burn to recipient
        // Burn the calculated amount
        msg!("Enhanced transfer: amount={}", amount);
        Ok(())
    }

    /// Process airdrop instruction
    fn process_airdrop(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        recipients: Vec<Pubkey>,
        amounts: Vec<u64>,
    ) -> ProgramResult {
        // Only operator or owner can airdrop
        // Validate vectors same length
        // Batch transfer to multiple recipients
        msg!("Airdrop: {} recipients, {} amounts", recipients.len(), amounts.len());
        Ok(())
    }

    /// Process enhanced stake instruction
    fn process_enhanced_stake(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        // Enhanced staking with lock time, penalties, etc.
        msg!("Enhanced stake: amount={}", amount);
        Ok(())
    }

    /// Process enhanced unstake instruction
    fn process_enhanced_unstake(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        // Enhanced unstaking with penalties for early withdrawal
        msg!("Enhanced unstake: amount={}", amount);
        Ok(())
    }

    /// Process enhanced claim rewards instruction
    fn process_enhanced_claim_rewards(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        // Enhanced reward claiming with time-based calculations
        msg!("Enhanced claim rewards");
        Ok(())
    }
}
