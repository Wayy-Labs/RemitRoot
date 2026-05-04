//! Voucher Token System (RVCH)
//!
//! Handles minting, burning, balance tracking, and transfer restrictions
//! for the RemitRoot voucher token. Vouchers can only be transferred to
//! approved vendor addresses.

use soroban_sdk::{Address, Bytes, BytesN, Env};

use crate::errors::Error;
use crate::events;
use crate::storage::{ContractAbi, DataKey};

// ---------------------------------------------------------------------------
// Voucher mint / burn
// ---------------------------------------------------------------------------

/// Mint `amount` voucher tokens to `farmer`.
/// Called internally after admin approves a farmer.
pub fn mint_voucher(env: &Env, farmer: &Address, amount: i128) {
    let key = DataKey::VoucherBalance(farmer.clone());
    let current: i128 = env.storage().persistent().get(&key).unwrap_or(0);
    env.storage().persistent().set(&key, &(current + amount));
}

/// Burn `amount` voucher tokens from `farmer`.
/// Only callable when the destination is an approved vendor.
pub fn burn_voucher(
    env: &Env,
    escrow_id: &BytesN<32>,
    farmer: &Address,
    vendor: &Address,
    amount: i128,
) -> Result<i128, Error> {
    // Vendor must be approved for transfer restriction
    let vendor_approved: bool = env
        .storage()
        .persistent()
        .get(&DataKey::VendorAddress(vendor.clone()))
        .unwrap_or(false);

    if !vendor_approved {
        return Err(Error::TransferRestricted);
    }

    let key = DataKey::VoucherBalance(farmer.clone());
    let balance: i128 = env.storage().persistent().get(&key).unwrap_or(0);

    if balance < amount {
        return Err(Error::InsufficientBalance);
    }

    env.storage().persistent().set(&key, &(balance - amount));
    events::voucher_burned(env, escrow_id, vendor, amount);

    Ok(amount)
}

/// Get voucher balance for an address.
pub fn get_voucher_balance(env: &Env, address: &Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::VoucherBalance(address.clone()))
        .unwrap_or(0)
}

// ---------------------------------------------------------------------------
// ABI / Schema Registry
// ---------------------------------------------------------------------------

/// Register a new ABI/schema for a contract.
/// Stores the schema bytes and initialises version to 1.
pub fn register_abi(
    env: &Env,
    contract_id: BytesN<32>,
    schema: Bytes,
) -> Result<u32, Error> {
    let version_key = DataKey::AbiVersion(contract_id.clone());
    if env.storage().persistent().has(&version_key) {
        // Already registered — use update_abi instead
        return Err(Error::VoucherAlreadyMinted); // reuse as "already registered"
    }

    let version: u32 = 1;
    let abi = ContractAbi {
        contract_id: contract_id.clone(),
        version,
        schema,
    };

    env.storage().persistent().set(&DataKey::ContractAbi(contract_id.clone()), &abi);
    env.storage().persistent().set(&version_key, &version);
    events::abi_registered(env, &contract_id, version);

    Ok(version)
}

/// Update an existing ABI/schema, incrementing the version.
pub fn update_abi(
    env: &Env,
    contract_id: BytesN<32>,
    schema: Bytes,
) -> Result<u32, Error> {
    let version_key = DataKey::AbiVersion(contract_id.clone());
    let old_version: u32 = env
        .storage()
        .persistent()
        .get(&version_key)
        .ok_or(Error::AbiNotFound)?;

    let new_version = old_version + 1;
    let abi = ContractAbi {
        contract_id: contract_id.clone(),
        version: new_version,
        schema,
    };

    env.storage().persistent().set(&DataKey::ContractAbi(contract_id.clone()), &abi);
    env.storage().persistent().set(&version_key, &new_version);
    events::abi_updated(env, &contract_id, old_version, new_version);

    Ok(new_version)
}

/// Retrieve the ABI/schema for a contract.
pub fn get_abi(env: &Env, contract_id: BytesN<32>) -> Result<ContractAbi, Error> {
    env.storage()
        .persistent()
        .get(&DataKey::ContractAbi(contract_id))
        .ok_or(Error::AbiNotFound)
}

/// Validate that a given schema matches the stored ABI for a contract.
pub fn validate_abi(env: &Env, contract_id: BytesN<32>, schema: &Bytes) -> Result<bool, Error> {
    let stored = get_abi(env, contract_id)?;
    Ok(&stored.schema == schema)
}
