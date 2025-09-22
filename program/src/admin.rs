//! Admin functionality for the E9th Token Program

use crate::{
    error::E9thTokenError,
    state::{deserialize_account_data, serialize_account_data, ProgramState},
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use spl_token::{
    instruction::{burn, mint_to},
};

pub struct AdminProcessor;

impl AdminProcessor {
    /// Initialize the program
    pub fn process_initialize(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        total_supply: u64,
        reward_rate: u16,
        min_stake_period: u64,
        max_stake_period: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let program_state_account = next_account_info(account_info_iter)?;
        let admin_account = next_account_info(account_info_iter)?;
        let mint_account = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;

        // Validate accounts
        if !program_state_account.is_writable {
            return Err(E9thTokenError::InvalidAccountData.into());
        }

        if !admin_account.is_signer {
            return Err(E9thTokenError::Unauthorized.into());
        }

        // Check if program state account is already initialized
        if program_state_account.data_is_empty() {
            // Create the program state account
            let rent = Rent::get()?;
            let space = ProgramState::LEN;
            let lamports = rent.minimum_balance(space);

            invoke(
                &system_instruction::create_account(
                    admin_account.key,
                    program_state_account.key,
                    lamports,
                    space as u64,
                    program_id,
                ),
                &[
                    admin_account.clone(),
                    program_state_account.clone(),
                    system_program.clone(),
                ],
            )?;

            // Initialize program state
            let program_state = ProgramState {
                admin: *admin_account.key,
                mint: *mint_account.key,
                total_supply,
                staking_enabled: true,
                reward_rate,
                min_stake_period,
                max_stake_period,
                total_staked: 0,
                bump: 0, // This would be calculated from the PDA derivation
            };

            serialize_account_data(program_state_account, &program_state)?;
        } else {
            return Err(E9thTokenError::AccountAlreadyInitialized.into());
        }

        msg!("Program initialized successfully");
        Ok(())
    }

    /// Mint tokens
    pub fn process_mint(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let program_state_account = next_account_info(account_info_iter)?;
        let admin_account = next_account_info(account_info_iter)?;
        let mint_account = next_account_info(account_info_iter)?;
        let destination_account = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;

        // Validate admin
        let program_state: ProgramState = deserialize_account_data(program_state_account)?;
        if program_state.admin != *admin_account.key {
            return Err(E9thTokenError::InvalidAdmin.into());
        }

        if !admin_account.is_signer {
            return Err(E9thTokenError::Unauthorized.into());
        }

        // Validate amount
        if amount == 0 {
            return Err(E9thTokenError::InvalidAmount.into());
        }

        // Mint tokens
        let mint_ix = mint_to(
            token_program.key,
            mint_account.key,
            destination_account.key,
            admin_account.key,
            &[],
            amount,
        )?;

        invoke(
            &mint_ix,
            &[
                mint_account.clone(),
                destination_account.clone(),
                admin_account.clone(),
                token_program.clone(),
            ],
        )?;

        // Update program state
        let mut program_state: ProgramState = deserialize_account_data(program_state_account)?;
        program_state.total_supply = program_state.total_supply.saturating_add(amount);
        serialize_account_data(program_state_account, &program_state)?;

        msg!("Minted {} tokens", amount);
        Ok(())
    }

    /// Burn tokens
    pub fn process_burn(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let program_state_account = next_account_info(account_info_iter)?;
        let admin_account = next_account_info(account_info_iter)?;
        let mint_account = next_account_info(account_info_iter)?;
        let source_account = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;

        // Validate admin
        let program_state: ProgramState = deserialize_account_data(program_state_account)?;
        if program_state.admin != *admin_account.key {
            return Err(E9thTokenError::InvalidAdmin.into());
        }

        if !admin_account.is_signer {
            return Err(E9thTokenError::Unauthorized.into());
        }

        // Validate amount
        if amount == 0 {
            return Err(E9thTokenError::InvalidAmount.into());
        }

        // Burn tokens
        let burn_ix = burn(
            token_program.key,
            source_account.key,
            mint_account.key,
            admin_account.key,
            &[],
            amount,
        )?;

        invoke(
            &burn_ix,
            &[
                source_account.clone(),
                mint_account.clone(),
                admin_account.clone(),
                token_program.clone(),
            ],
        )?;

        // Update program state
        let mut program_state: ProgramState = deserialize_account_data(program_state_account)?;
        program_state.total_supply = program_state.total_supply.saturating_sub(amount);
        serialize_account_data(program_state_account, &program_state)?;

        msg!("Burned {} tokens", amount);
        Ok(())
    }

    /// Update program settings
    pub fn process_update_settings(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        reward_rate: Option<u16>,
        min_stake_period: Option<u64>,
        max_stake_period: Option<u64>,
        staking_enabled: Option<bool>,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let program_state_account = next_account_info(account_info_iter)?;
        let admin_account = next_account_info(account_info_iter)?;

        // Validate admin
        let mut program_state: ProgramState = deserialize_account_data(program_state_account)?;
        if program_state.admin != *admin_account.key {
            return Err(E9thTokenError::InvalidAdmin.into());
        }

        if !admin_account.is_signer {
            return Err(E9thTokenError::Unauthorized.into());
        }

        // Update settings
        if let Some(rate) = reward_rate {
            program_state.reward_rate = rate;
        }
        if let Some(period) = min_stake_period {
            if period < 1 {
                return Err(E9thTokenError::StakePeriodTooShort.into());
            }
            program_state.min_stake_period = period;
        }
        if let Some(period) = max_stake_period {
            if period > 365 {
                return Err(E9thTokenError::StakePeriodTooLong.into());
            }
            program_state.max_stake_period = period;
        }
        if let Some(enabled) = staking_enabled {
            program_state.staking_enabled = enabled;
        }

        // Validate settings
        if program_state.min_stake_period > program_state.max_stake_period {
            return Err(E9thTokenError::InvalidStakePeriod.into());
        }

        serialize_account_data(program_state_account, &program_state)?;

        msg!("Settings updated successfully");
        Ok(())
    }

    /// Transfer admin authority
    pub fn process_transfer_admin(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        new_admin: Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let program_state_account = next_account_info(account_info_iter)?;
        let current_admin_account = next_account_info(account_info_iter)?;

        // Validate current admin
        let mut program_state: ProgramState = deserialize_account_data(program_state_account)?;
        if program_state.admin != *current_admin_account.key {
            return Err(E9thTokenError::InvalidAdmin.into());
        }

        if !current_admin_account.is_signer {
            return Err(E9thTokenError::Unauthorized.into());
        }

        // Transfer admin
        program_state.admin = new_admin;
        serialize_account_data(program_state_account, &program_state)?;

        msg!("Admin transferred to: {}", new_admin);
        Ok(())
    }
}
