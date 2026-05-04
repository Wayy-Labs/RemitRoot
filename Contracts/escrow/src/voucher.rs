use soroban_sdk::{Address, BytesN, Env};

use crate::errors::Error;
use crate::events;
use crate::storage::{DataKey, EscrowRecord, EscrowState};

/// Mint voucher balance for a farmer (called internally after approve_farmer).
pub fn mint_voucher(env: &Env, farmer: &Address, amount: i128) {
    let key = DataKey::VoucherBalance(farmer.clone());
    let current: i128 = env.storage().persistent().get(&key).unwrap_or(0);
    env.storage().persistent().set(&key, &(current + amount));
}

/// Burn voucher — vendor redeems. Returns the amount burned.
pub fn burn_voucher(env: &Env, escrow_id: &BytesN<32>, farmer: &Address, amount: i128) -> Result<i128, Error> {
    let key = DataKey::VoucherBalance(farmer.clone());
    let balance: i128 = env.storage().persistent().get(&key).unwrap_or(0);

    if balance < amount {
        return Err(Error::InvalidAmount);
    }

    env.storage().persistent().set(&key, &(balance - amount));
    Ok(amount)
}

/// Get voucher balance for an address.
pub fn get_voucher_balance(env: &Env, address: &Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::VoucherBalance(address.clone()))
        .unwrap_or(0)
}
