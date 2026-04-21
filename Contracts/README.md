# Smart Contracts 🔗

> **Soroban smart contracts for RemitRoot** — the trustless heart of cross-border farm input financing on Stellar.

---

## Table of Contents

- [Overview](#overview)
- [Contract Architecture](#contract-architecture)
- [Core Contract](#core-contract)
- [State Machine](#state-machine)
- [Key Functions](#key-functions)
- [Tech Stack](#tech-stack)
- [Setup](#setup)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Environment Variables](#environment-variables)
  - [Building](#building)
  - [Testing](#testing)
- [Deployment](#deployment)
- [Security Considerations](#security-considerations)
- [Gas Optimization](#gas-optimization)
- [Audit Checklist](#audit-checklist)

---

## Overview

The smart contracts implement the core escrow logic that enables RemitRoot's trustless financing system. Built with Soroban (Stellar's smart contract platform), these contracts handle:

- Fund locking and release
- Voucher token minting and burning
- Repayment processing
- State management and validation
- Access control and permissions

All contract state is stored on-chain, ensuring transparency and immutability of the financing process.

---

## Contract Architecture

### 📦 `contracts/escrow`

The core Soroban smart contract written in Rust. This is the trustless heart of RemitRoot.

```
packages/contracts/escrow/
├── src/
│   ├── lib.rs               # Contract entry point
│   ├── escrow.rs            # Core escrow logic
│   ├── voucher.rs           # Voucher token mint/burn
│   ├── repayment.rs         # Repayment stream logic
│   ├── storage.rs           # Persistent state types
│   ├── events.rs            # Contract events
│   ├── access_control.rs    # Permission management
│   └── errors.rs            # Contract error codes
├── tests/
│   ├── integration.rs       # Full flow integration tests
│   ├── unit_tests.rs        # Individual function tests
│   └── security_tests.rs    # Security edge cases
├── scripts/
│   ├── deploy.ts            # Deployment script
│   └── interact.ts          # Interaction helpers
└── Cargo.toml
```

---

## State Machine

The escrow contract follows a strict state machine to ensure proper fund flow:

| State | Trigger | Description |
|---|---|---|
| `Created` | `fund()` | Sender deposits USDC, escrow created |
| `Funded` | `approve_farmer()` | Admin approves farmer, escrow locks |
| `VoucherMinted` | `mint_voucher()` | RVCH token sent to farmer's wallet |
| `Redeemed` | `redeem_voucher()` | Vendor burns token, goods delivered |
| `Repaying` | `trigger_repay()` | Oracle signals harvest season |
| `Closed` | `repay()` / `default()` | Escrow fully settled or timed out |

### State Transitions

```
fund() ──▶ Funded ──▶ approve_farmer() ──▶ VoucherMinted
                                                 │
                                          redeem_voucher()
                                                 │
                                            Redeemed
                                                 │
                                          trigger_repay()  ◀── oracle
                                                 │
                                            Repaying
                                           ╱         ╲
                                     repay()       default()
                                        │               │
                                     Repaid         Defaulted
```

---

## Key Functions

### Core Escrow Operations

```rust
// Sender locks funds for a specific farmer + vendor + season
pub fn fund(env: Env, sender: Address, vendor_id: BytesN<32>,
            crop_season: Symbol, amount: i128) -> Result<BytesN<32>, Error>

// Admin/DAO approves farmer and mints voucher
pub fn approve_farmer(env: Env, escrow_id: BytesN<32>,
                      farmer: Address) -> Result<(), Error>

// Vendor calls this to burn voucher and release USDC to themselves
pub fn redeem_voucher(env: Env, escrow_id: BytesN<32>,
                      vendor: Address) -> Result<(), Error>

// Oracle triggers repayment window after harvest
pub fn trigger_repay(env: Env, escrow_id: BytesN<32>) -> Result<(), Error>

// Farmer calls to make a partial repayment
pub fn repay(env: Env, escrow_id: BytesN<32>,
             farmer: Address, amount: i128) -> Result<(), Error>

// Cancel and refund if no farmer approved within timeout
pub fn cancel(env: Env, escrow_id: BytesN<32>) -> Result<(), Error>
```

### Voucher Token Management

```rust
// Mint voucher tokens to farmer
pub fn mint_voucher(env: Env, escrow_id: BytesN<32>,
                    farmer: Address, amount: i128) -> Result<(), Error>

// Burn voucher tokens (called by vendor during redemption)
pub fn burn_voucher(env: Env, escrow_id: BytesN<32>,
                    vendor: Address, amount: i128) -> Result<(), Error>

// Check voucher balance
pub fn get_voucher_balance(env: Env, account: Address) -> Result<i128, Error>
```

### Access Control

```rust
// Set admin address (contract owner only)
pub fn set_admin(env: Env, new_admin: Address) -> Result<(), Error>

// Set oracle address (admin only)
pub fn set_oracle(env: Env, new_oracle: Address) -> Result<(), Error>

// Add approved vendor (admin only)
pub fn approve_vendor(env: Env, vendor: Address) -> Result<(), Error>

// Remove vendor (admin only)
pub fn remove_vendor(env: Env, vendor: Address) -> Result<(), Error>
```

---

## Tech Stack

| Layer | Technology |
|---|---|
| Language | Rust |
| Platform | Soroban (Stellar Smart Contracts) |
| Testing | Stellar Soroban SDK Test Framework |
| Build | Cargo + Soroban CLI |
| Deployment | Stellar CLI + Stellar Testnet/Mainnet |
| Verification | Stellar Explorer |
| Gas Optimization | Soroban Gas Metering |

---

## Setup

### Prerequisites

- **Rust** 1.70+ with `cargo`
- **Stellar CLI** (`stellar` — includes Soroban CLI)
- **Rust Wasm target** for contract compilation

### Installation

```bash
# Navigate to contracts directory
cd Contracts

# Install the Stellar CLI
cargo install --locked stellar-cli --features opt

# Install Rust Wasm target for Soroban contract compilation
rustup target add wasm32-unknown-unknown

# Install contract dependencies
cargo build
```

### Environment Variables

Create a `.env` file in the contracts directory:

```env
# Stellar network
STELLAR_NETWORK=testnet
STELLAR_RPC_URL=https://soroban-testnet.stellar.org
STELLAR_HORIZON_URL=https://horizon-testnet.stellar.org
STELLAR_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"

# Deployment
DEPLOYER_SECRET_KEY=your-deployer-secret-key
DEPLOYER_PUBLIC_KEY=your-deployer-public-key

# Contract configuration
ADMIN_ADDRESS=your-admin-address
ORACLE_ADDRESS=your-oracle-address
APPROVAL_TIMEOUT_LEDGERS=604800  # 7 days in seconds
```

### Building

```bash
# Build the contract for testing
cargo build

# Build for deployment (optimized)
stellar contract build

# Check contract size and gas usage
stellar contract size
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_fund_flow

# Run integration tests
cargo test --test integration

# Run with gas profiling
cargo test --features gas-profiling
```

**Test coverage targets:**

- `fund()` — valid deposit, duplicate escrow rejection
- `approve_farmer()` — auth check, state transition
- `redeem_voucher()` — wrong vendor rejection, double-redeem protection
- `repay()` — partial repayments, overpayment rejection
- `cancel()` — before/after timeout, refund amount correctness
- `default()` — oracle-triggered, funds returned to sender

---

## Deployment

### Testnet Deployment

```bash
# Deploy contract
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/remitroot_escrow.wasm \
  --source your-deployer-key \
  --network testnet

# Initialize contract (set admin, oracle, etc.)
stellar contract invoke \
  --id $CONTRACT_ID \
  --function initialize \
  --args admin:$ADMIN_ADDRESS oracle:$ORACLE_ADDRESS \
  --source your-deployer-key \
  --network testnet

# Verify contract on Stellar Explorer
stellar contract info --id $CONTRACT_ID --network testnet
```

### Mainnet Deployment

```bash
# Requires a funded mainnet deployer account
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/remitroot_escrow.wasm \
  --source your-mainnet-deployer-key \
  --network mainnet

# Initialize with production addresses
stellar contract invoke \
  --id $MAINNET_CONTRACT_ID \
  --function initialize \
  --args admin:$PROD_ADMIN_ADDRESS oracle:$PROD_ORACLE_ADDRESS \
  --source your-mainnet-deployer-key \
  --network mainnet
```

### Contract Verification

```bash
# Verify source code on Stellar Explorer
stellar contract verify \
  --id $CONTRACT_ID \
  --source-hash $(stellar contract compute-hash --wasm target/wasm32-unknown-unknown/release/remitroot_escrow.wasm) \
  --network testnet
```

---

## Security Considerations

### Access Control

**Role-Based Permissions:**
- **Admin**: Can set oracle, approve vendors, modify contract parameters
- **Oracle**: Can trigger repayment windows
- **Vendor**: Can redeem vouchers (must be pre-approved)
- **Sender**: Can fund escrows and receive repayments
- **Farmer**: Can receive vouchers and make repayments

**Critical Functions Protected:**
- `initialize()` - Contract setup (admin only)
- `set_admin()` - Admin transfer (admin only)
- `set_oracle()` - Oracle management (admin only)
- `approve_vendor()` - Vendor whitelisting (admin only)
- `trigger_repay()` - Repayment window (oracle only)

### State Validation

**Invariant Checks:**
- Escrow amounts must be positive
- Voucher amounts cannot exceed funded amounts
- Repayments cannot exceed total owed
- State transitions must follow the defined machine
- Timeouts must be respected

**Reentrancy Protection:**
- All state changes happen before external calls
- No external dependencies in critical paths
- Atomic operations where possible

### Economic Security

**Slashing Conditions:**
- Oracle misbehavior can be challenged
- Vendor fraud results in blacklisting
- Double-spending attempts are prevented by state machine

**Fee Structure:**
- Protocol fees collected during redemption
- Oracle fees for repayment processing
- Gas costs optimized for frequent operations

---

## Gas Optimization

### Storage Optimization

**Efficient Data Structures:**
- Use `BytesN<32>` for IDs instead of strings
- Pack related data into structs
- Use `Symbol` for enums instead of strings
- Implement lazy loading for large datasets

**Storage Patterns:**
- Store only essential data on-chain
- Use events for off-chain indexing
- Implement data pruning where possible

### Computation Optimization

**Algorithm Efficiency:**
- Use O(1) lookups with persistent storage
- Minimize loops and complex calculations
- Batch operations where possible
- Cache frequently accessed data

**Gas Metering:**
- Profile all functions for gas usage
- Optimize hot paths first
- Use `env.require_auth()` for cheap access control
- Implement gas limits for user operations

---

## Audit Checklist

### Code Quality

- [ ] All functions have proper error handling
- [ ] No unchecked arithmetic operations
- [ ] All external calls are validated
- [ ] State transitions are atomic
- [ ] No integer overflows/underflows

### Security

- [ ] Access control is properly implemented
- [ ] Reentrancy attacks are prevented
- [ ] Time-dependent operations use block timestamps
- [ ] Admin functions have proper safeguards
- [ ] Emergency pause mechanisms exist

### Economic Logic

- [ ] Fund flows match the specification
- [ ] Fee calculations are correct
- [ ] Repayment logic handles edge cases
- [ ] Timeout mechanisms work correctly
- [ ] No money can be stuck in contracts

### Testing

- [ ] All function paths are tested
- [ ] Edge cases are covered
- [ ] Integration tests pass
- [ ] Gas usage is within limits
- [ ] Performance benchmarks exist

### Deployment

- [ ] Contract is verified on Explorer
- [ ] Initialization parameters are correct
- [ ] Admin keys are securely stored
- [ ] Monitoring is set up
- [ ] Upgrade path is documented

---

## Error Codes

| Code | Name | Meaning |
|---|---|---|
| `1` | `AlreadyFunded` | `fund()` called on an existing escrow |
| `2` | `NotFunded` | Action requires `Funded` state |
| `3` | `Unauthorized` | Caller is not the expected party |
| `4` | `VoucherAlreadyMinted` | Cannot mint twice |
| `5` | `NotRedeemed` | Cannot trigger repay before redemption |
| `6` | `RepaymentComplete` | Surplus repayment rejected |
| `7` | `NotExpired` | Cannot cancel before timeout |
| `8` | `InvalidAmount` | Amount must be positive |
| `9` | `InvalidVendor` | Vendor not approved |
| `10` | `InsufficientBalance` | Not enough funds for operation |

---

## Contributing

1. Follow Rust best practices and conventions
2. Write comprehensive tests for new functions
3. Document all public interfaces
4. Consider gas implications of changes
5. Run security analysis tools
6. Update this README for architectural changes

---

## License

MIT © 2026 RemitRoot Contributors
