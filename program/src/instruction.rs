//! Instruction definitions for the E9th Token Program

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Enhanced instructions for the E9th Token Program
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum E9thInstruction {
    /// Initialize the config account
    Initialize {
        owner: Pubkey,
        operator: Pubkey,
        treasury: Pubkey,
        burn_rate_basis_points: u16,
    },
    /// Pause or unpause token transfers
    SetPause {
        pause: bool,
    },
    /// Add or remove from blacklist
    ModifyBlacklist {
        account: Pubkey,
        add: bool,
    },
    /// Transfer tokens (with burn rate / deflation logic)
    Transfer {
        amount: u64,
    },
    /// Batch Airdrop: airdrop to multiple accounts
    Airdrop {
        recipients: Vec<Pubkey>,
        amounts: Vec<u64>,
    },
    /// Stake some tokens
    Stake {
        amount: u64,
    },
    /// Unstake
    Unstake {
        amount: u64,
    },
    /// Claim staking rewards
    ClaimRewards,
    /// Legacy instructions for backward compatibility
    LegacyInitialize {
        total_supply: u64,
        reward_rate: u16,
        min_stake_period: u64,
        max_stake_period: u64,
    },
    LegacyMint {
        amount: u64,
    },
    LegacyBurn {
        amount: u64,
    },
    LegacyStake {
        amount: u64,
        period: u64,
    },
    LegacyUnstake,
    LegacyClaimRewards,
    LegacyUpdateSettings {
        reward_rate: Option<u16>,
        min_stake_period: Option<u64>,
        max_stake_period: Option<u64>,
        staking_enabled: Option<bool>,
    },
    LegacyTransferAdmin {
        new_admin: Pubkey,
    },
}

/// Instructions supported by the E9th Token Program (legacy)
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum E9thTokenInstruction {
    /// Initialize the program
    /// Accounts:
    /// 0. [writable] Program state account
    /// 1. [signer] Admin authority
    /// 2. [writable] Token mint account
    /// 3. [] System program
    /// 4. [] Token program
    Initialize {
        total_supply: u64,
        reward_rate: u16,
        min_stake_period: u64,
        max_stake_period: u64,
    },

    /// Mint tokens to a user
    /// Accounts:
    /// 0. [writable] Program state account
    /// 1. [signer] Admin authority
    /// 2. [writable] Token mint account
    /// 3. [writable] Destination token account
    /// 4. [] Token program
    Mint {
        amount: u64,
    },

    /// Burn tokens from a user
    /// Accounts:
    /// 0. [writable] Program state account
    /// 1. [signer] Admin authority
    /// 2. [writable] Token mint account
    /// 3. [writable] Source token account
    /// 4. [] Token program
    Burn {
        amount: u64,
    },

    /// Stake tokens
    /// Accounts:
    /// 0. [writable] Program state account
    /// 1. [writable] Stake account
    /// 2. [signer] User
    /// 3. [writable] User's token account
    /// 4. [] Token program
    Stake {
        amount: u64,
        period: u64,
    },

    /// Unstake tokens
    /// Accounts:
    /// 0. [writable] Program state account
    /// 1. [writable] Stake account
    /// 2. [signer] User
    /// 3. [writable] User's token account
    /// 4. [] Token program
    Unstake,

    /// Claim rewards
    /// Accounts:
    /// 0. [writable] Program state account
    /// 1. [writable] Stake account
    /// 2. [signer] User
    /// 3. [writable] User's token account
    /// 4. [] Token program
    ClaimRewards,

    /// Update program settings (admin only)
    /// Accounts:
    /// 0. [writable] Program state account
    /// 1. [signer] Admin authority
    UpdateSettings {
        reward_rate: Option<u16>,
        min_stake_period: Option<u64>,
        max_stake_period: Option<u64>,
        staking_enabled: Option<bool>,
    },

    /// Transfer admin authority
    /// Accounts:
    /// 0. [writable] Program state account
    /// 1. [signer] Current admin
    /// 2. [] New admin
    TransferAdmin {
        new_admin: solana_program::pubkey::Pubkey,
    },
}

impl E9thInstruction {
    /// Unpack a byte buffer into a [E9thInstruction](enum.E9thInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            0 => {
                let (owner_bytes, rest) = rest.split_at(32);
                let (operator_bytes, rest) = rest.split_at(32);
                let (treasury_bytes, rest) = rest.split_at(32);
                let (burn_rate, _) = Self::unpack_u16(rest)?;
                
                let owner = Pubkey::try_from(owner_bytes).map_err(|_| ProgramError::InvalidInstructionData)?;
                let operator = Pubkey::try_from(operator_bytes).map_err(|_| ProgramError::InvalidInstructionData)?;
                let treasury = Pubkey::try_from(treasury_bytes).map_err(|_| ProgramError::InvalidInstructionData)?;
                
                Self::Initialize {
                    owner,
                    operator,
                    treasury,
                    burn_rate_basis_points: burn_rate,
                }
            }
            1 => {
                let (pause, _) = Self::unpack_u8(rest)?;
                Self::SetPause { pause: pause != 0 }
            }
            2 => {
                let (account_bytes, rest) = rest.split_at(32);
                let (add, _) = Self::unpack_u8(rest)?;
                let account = Pubkey::try_from(account_bytes).map_err(|_| ProgramError::InvalidInstructionData)?;
                Self::ModifyBlacklist { account, add: add != 0 }
            }
            3 => {
                let (amount, _) = Self::unpack_u64(rest)?;
                Self::Transfer { amount }
            }
            4 => {
                // For airdrop, we need to handle variable length vectors
                // This is simplified - in practice, you'd need more complex parsing
                Self::Airdrop {
                    recipients: Vec::new(),
                    amounts: Vec::new(),
                }
            }
            5 => {
                let (amount, _) = Self::unpack_u64(rest)?;
                Self::Stake { amount }
            }
            6 => {
                let (amount, _) = Self::unpack_u64(rest)?;
                Self::Unstake { amount }
            }
            7 => Self::ClaimRewards,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    /// Pack a [E9thInstruction](enum.E9thInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            Self::Initialize { owner, operator, treasury, burn_rate_basis_points } => {
                buf.push(0);
                buf.extend_from_slice(owner.as_ref());
                buf.extend_from_slice(operator.as_ref());
                buf.extend_from_slice(treasury.as_ref());
                buf.extend_from_slice(&burn_rate_basis_points.to_le_bytes());
            }
            Self::SetPause { pause } => {
                buf.push(1);
                buf.push(if *pause { 1 } else { 0 });
            }
            Self::ModifyBlacklist { account, add } => {
                buf.push(2);
                buf.extend_from_slice(account.as_ref());
                buf.push(if *add { 1 } else { 0 });
            }
            Self::Transfer { amount } => {
                buf.push(3);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::Airdrop { recipients, amounts } => {
                buf.push(4);
                // Simplified - in practice, you'd serialize the vectors properly
                buf.extend_from_slice(&recipients.len().to_le_bytes());
                for recipient in recipients {
                    buf.extend_from_slice(recipient.as_ref());
                }
                buf.extend_from_slice(&amounts.len().to_le_bytes());
                for amount in amounts {
                    buf.extend_from_slice(&amount.to_le_bytes());
                }
            }
            Self::Stake { amount } => {
                buf.push(5);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::Unstake { amount } => {
                buf.push(6);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::ClaimRewards => {
                buf.push(7);
            }
            _ => {
                // Handle legacy instructions by delegating to the old enum
                return match self.to_legacy() {
                    Ok(legacy_instr) => legacy_instr.pack(),
                    Err(_) => Vec::new(), // Return empty vector on error
                };
            }
        }
        buf
    }

    /// Convert to legacy instruction for backward compatibility
    fn to_legacy(&self) -> Result<E9thTokenInstruction, ProgramError> {
        match self {
            Self::LegacyInitialize { total_supply, reward_rate, min_stake_period, max_stake_period } => {
                Ok(E9thTokenInstruction::Initialize {
                    total_supply: *total_supply,
                    reward_rate: *reward_rate,
                    min_stake_period: *min_stake_period,
                    max_stake_period: *max_stake_period,
                })
            }
            Self::LegacyMint { amount } => {
                Ok(E9thTokenInstruction::Mint { amount: *amount })
            }
            Self::LegacyBurn { amount } => {
                Ok(E9thTokenInstruction::Burn { amount: *amount })
            }
            Self::LegacyStake { amount, period } => {
                Ok(E9thTokenInstruction::Stake { amount: *amount, period: *period })
            }
            Self::LegacyUnstake => {
                Ok(E9thTokenInstruction::Unstake)
            }
            Self::LegacyClaimRewards => {
                Ok(E9thTokenInstruction::ClaimRewards)
            }
            Self::LegacyUpdateSettings { reward_rate, min_stake_period, max_stake_period, staking_enabled } => {
                Ok(E9thTokenInstruction::UpdateSettings {
                    reward_rate: *reward_rate,
                    min_stake_period: *min_stake_period,
                    max_stake_period: *max_stake_period,
                    staking_enabled: *staking_enabled,
                })
            }
            Self::LegacyTransferAdmin { new_admin } => {
                Ok(E9thTokenInstruction::TransferAdmin { new_admin: *new_admin })
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }

    fn unpack_u8(input: &[u8]) -> Result<(u8, &[u8]), ProgramError> {
        if input.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }
        Ok((input[0], &input[1..]))
    }

    fn unpack_u16(input: &[u8]) -> Result<(u16, &[u8]), ProgramError> {
        if input.len() < 2 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (bytes, rest) = input.split_at(2);
        let value = u16::from_le_bytes([bytes[0], bytes[1]]);
        Ok((value, rest))
    }

    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() < 8 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (bytes, rest) = input.split_at(8);
        let value = u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        Ok((value, rest))
    }
}

impl E9thTokenInstruction {
    /// Unpack a byte buffer into a [E9thTokenInstruction](enum.E9thTokenInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            0 => {
                let (total_supply, rest) = Self::unpack_u64(rest)?;
                let (reward_rate, rest) = Self::unpack_u16(rest)?;
                let (min_stake_period, rest) = Self::unpack_u64(rest)?;
                let (max_stake_period, _) = Self::unpack_u64(rest)?;
                Self::Initialize {
                    total_supply,
                    reward_rate,
                    min_stake_period,
                    max_stake_period,
                }
            }
            1 => {
                let (amount, _) = Self::unpack_u64(rest)?;
                Self::Mint { amount }
            }
            2 => {
                let (amount, _) = Self::unpack_u64(rest)?;
                Self::Burn { amount }
            }
            3 => {
                let (amount, rest) = Self::unpack_u64(rest)?;
                let (period, _) = Self::unpack_u64(rest)?;
                Self::Stake { amount, period }
            }
            4 => Self::Unstake,
            5 => Self::ClaimRewards,
            6 => {
                let mut reward_rate = None;
                let mut min_stake_period = None;
                let mut max_stake_period = None;
                let mut staking_enabled = None;
                
                let mut data = rest;
                while !data.is_empty() {
                    let (field_type, rest) = data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
                    data = rest;
                    
                    match field_type {
                        0 => {
                            let (value, rest) = Self::unpack_u16(data)?;
                            reward_rate = Some(value);
                            data = rest;
                        }
                        1 => {
                            let (value, rest) = Self::unpack_u64(data)?;
                            min_stake_period = Some(value);
                            data = rest;
                        }
                        2 => {
                            let (value, rest) = Self::unpack_u64(data)?;
                            max_stake_period = Some(value);
                            data = rest;
                        }
                        3 => {
                            let (value, rest) = data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
                            staking_enabled = Some(*value != 0);
                            data = rest;
                        }
                        _ => return Err(ProgramError::InvalidInstructionData),
                    }
                }
                
                Self::UpdateSettings {
                    reward_rate,
                    min_stake_period,
                    max_stake_period,
                    staking_enabled,
                }
            }
            7 => {
                let (new_admin_bytes, _) = rest.split_at(32);
                let new_admin = solana_program::pubkey::Pubkey::try_from(new_admin_bytes)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Self::TransferAdmin { new_admin }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    /// Pack a [E9thTokenInstruction](enum.E9thTokenInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            Self::Initialize {
                total_supply,
                reward_rate,
                min_stake_period,
                max_stake_period,
            } => {
                buf.push(0);
                buf.extend_from_slice(&total_supply.to_le_bytes());
                buf.extend_from_slice(&reward_rate.to_le_bytes());
                buf.extend_from_slice(&min_stake_period.to_le_bytes());
                buf.extend_from_slice(&max_stake_period.to_le_bytes());
            }
            Self::Mint { amount } => {
                buf.push(1);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::Burn { amount } => {
                buf.push(2);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::Stake { amount, period } => {
                buf.push(3);
                buf.extend_from_slice(&amount.to_le_bytes());
                buf.extend_from_slice(&period.to_le_bytes());
            }
            Self::Unstake => {
                buf.push(4);
            }
            Self::ClaimRewards => {
                buf.push(5);
            }
            Self::UpdateSettings {
                reward_rate,
                min_stake_period,
                max_stake_period,
                staking_enabled,
            } => {
                buf.push(6);
                if let Some(rate) = reward_rate {
                    buf.push(0);
                    buf.extend_from_slice(&rate.to_le_bytes());
                }
                if let Some(period) = min_stake_period {
                    buf.push(1);
                    buf.extend_from_slice(&period.to_le_bytes());
                }
                if let Some(period) = max_stake_period {
                    buf.push(2);
                    buf.extend_from_slice(&period.to_le_bytes());
                }
                if let Some(enabled) = staking_enabled {
                    buf.push(3);
                    buf.push(if *enabled { 1 } else { 0 });
                }
            }
            Self::TransferAdmin { new_admin } => {
                buf.push(7);
                buf.extend_from_slice(new_admin.as_ref());
            }
        }
        buf
    }

    fn unpack_u8(input: &[u8]) -> Result<(u8, &[u8]), ProgramError> {
        if input.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }
        Ok((input[0], &input[1..]))
    }

    fn unpack_u16(input: &[u8]) -> Result<(u16, &[u8]), ProgramError> {
        if input.len() < 2 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (bytes, rest) = input.split_at(2);
        let value = u16::from_le_bytes([bytes[0], bytes[1]]);
        Ok((value, rest))
    }

    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() < 8 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (bytes, rest) = input.split_at(8);
        let value = u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        Ok((value, rest))
    }
}
