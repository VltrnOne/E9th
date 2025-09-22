# E9th Token Program

A Solana program for creating and managing custom tokens with staking functionality.

## Features

### Core Token Management
- **Token Creation**: Initialize custom tokens with configurable parameters
- **Mint/Burn**: Create and destroy tokens with admin controls
- **Deflationary Mechanism**: Built-in burn rate for transfers (configurable basis points)
- **Pause Functionality**: Emergency pause/unpause for all token transfers

### Enhanced Security & Control
- **Blacklist System**: Add/remove accounts from blacklist to prevent transfers
- **Multi-Role Access**: Separate owner (multisig) and operator roles
- **Treasury Management**: Dedicated treasury account for rewards and operations

### Advanced Staking System
- **Flexible Staking**: Stake tokens with configurable lock periods
- **Time-Based Rewards**: Calculate rewards based on staking duration
- **Early Unstake Penalties**: Lock time enforcement with penalty system
- **Batch Operations**: Efficient airdrop and batch transfer capabilities

### Administrative Features
- **Role-Based Access**: Owner controls critical functions, operator handles day-to-day
- **Configurable Parameters**: Adjustable burn rates, reward rates, staking periods
- **Backward Compatibility**: Legacy instruction support for existing integrations

## Program Structure

```
program/
├── src/
│   ├── lib.rs              # Main library file with module declarations
│   ├── entrypoint.rs       # Program entry point
│   ├── processor.rs        # Main instruction processor
│   ├── instruction.rs      # Instruction definitions and serialization
│   ├── state.rs           # Account state definitions
│   ├── error.rs           # Custom error types
│   ├── admin.rs           # Admin functionality
│   └── stake.rs           # Staking functionality
├── tests/
│   └── integration_tests.rs # Integration tests
├── Cargo.toml             # Dependencies and configuration
└── README.md              # This file
```

## Instructions

### Enhanced Instructions

#### Initialize
Initialize the enhanced program with multi-role access and deflationary features.

**Accounts:**
- Token config account (writable)
- Blacklist account (writable)
- Owner authority (signer)
- Token mint account (writable)
- System program
- Token program

**Parameters:**
- `owner`: Multisig or super admin public key
- `operator`: Pause, blacklist, airdrop operations public key
- `treasury`: Holds reward pool and operations public key
- `burn_rate_basis_points`: Transfer burn rate (e.g., 100 = 1%)

#### SetPause
Pause or unpause token transfers (owner only).

**Accounts:**
- Token config account (writable)
- Owner authority (signer)

**Parameters:**
- `pause`: Boolean to pause (true) or unpause (false)

#### ModifyBlacklist
Add or remove accounts from blacklist (operator or owner).

**Accounts:**
- Blacklist account (writable)
- Operator/Owner authority (signer)

**Parameters:**
- `account`: Public key to add/remove
- `add`: Boolean to add (true) or remove (false)

#### Transfer
Transfer tokens with deflationary burn mechanism.

**Accounts:**
- Token config account (readonly)
- Blacklist account (readonly)
- Source token account (writable)
- Destination token account (writable)
- Token program

**Parameters:**
- `amount`: Amount to transfer

#### Airdrop
Batch airdrop to multiple accounts (operator or owner).

**Accounts:**
- Token config account (readonly)
- Treasury token account (writable)
- Multiple recipient token accounts (writable)
- Token program

**Parameters:**
- `recipients`: Vector of recipient public keys
- `amounts`: Vector of amounts (must match recipients length)

#### Enhanced Stake
Stake tokens with lock time and penalty system.

**Accounts:**
- Token config account (readonly)
- Stake entry account (writable)
- Staker token account (writable)
- Staking vault (writable)
- Token program

**Parameters:**
- `amount`: Amount to stake

#### Enhanced Unstake
Unstake tokens with early withdrawal penalties.

**Accounts:**
- Token config account (readonly)
- Stake entry account (writable)
- Staker token account (writable)
- Staking vault (writable)
- Token program

**Parameters:**
- `amount`: Amount to unstake

#### ClaimRewards
Claim staking rewards based on time-based calculations.

**Accounts:**
- Token config account (readonly)
- Stake entry account (writable)
- Staker token account (writable)
- Treasury token account (writable)
- Token program

### Legacy Instructions

#### Legacy Initialize
Initialize the program with initial settings (backward compatibility).

**Accounts:**
- Program state account (writable)
- Admin authority (signer)
- Token mint account (writable)
- System program
- Token program

**Parameters:**
- `total_supply`: Initial token supply
- `reward_rate`: Reward rate per epoch (basis points)
- `min_stake_period`: Minimum staking period (epochs)
- `max_stake_period`: Maximum staking period (epochs)

### Mint
Mint new tokens (admin only).

**Accounts:**
- Program state account (writable)
- Admin authority (signer)
- Token mint account (writable)
- Destination token account (writable)
- Token program

**Parameters:**
- `amount`: Amount to mint

### Burn
Burn tokens (admin only).

**Accounts:**
- Program state account (writable)
- Admin authority (signer)
- Token mint account (writable)
- Source token account (writable)
- Token program

**Parameters:**
- `amount`: Amount to burn

### Stake
Stake tokens for rewards.

**Accounts:**
- Program state account (writable)
- Stake account (writable)
- User (signer)
- User's token account (writable)
- Token program

**Parameters:**
- `amount`: Amount to stake
- `period`: Staking period (epochs)

### Unstake
Unstake tokens after the staking period.

**Accounts:**
- Program state account (writable)
- Stake account (writable)
- User (signer)
- User's token account (writable)
- Token program

### Claim Rewards
Claim staking rewards.

**Accounts:**
- Program state account (writable)
- Stake account (writable)
- User (signer)
- User's token account (writable)
- Token program

### Update Settings
Update program settings (admin only).

**Accounts:**
- Program state account (writable)
- Admin authority (signer)

**Parameters:**
- `reward_rate`: New reward rate (optional)
- `min_stake_period`: New minimum staking period (optional)
- `max_stake_period`: New maximum staking period (optional)
- `staking_enabled`: Enable/disable staking (optional)

### Transfer Admin
Transfer admin authority to a new address.

**Accounts:**
- Program state account (writable)
- Current admin (signer)

**Parameters:**
- `new_admin`: New admin public key

## Building and Testing

### Prerequisites
- Rust 1.70+
- Solana CLI tools

### Build
```bash
cargo build-bpf
```

### Test
```bash
cargo test
```

### Deploy
```bash
solana program deploy target/deploy/e9th_token_program.so
```

## Security Considerations

- All admin functions require proper authorization
- Staking periods are validated against configured limits
- Reward calculations are protected against overflow
- Account ownership is verified for all operations

## License

This project is licensed under the MIT License.
