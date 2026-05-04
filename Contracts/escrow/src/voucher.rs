use soroban_sdk::{Address, Env};
use crate::storage::DataKey;

pub fn mint_voucher(env: &Env, farmer: &Address, amount: i128) {
    let key = DataKey::VoucherBalance(farmer.clone());
    let current: i128 = env.storage().persistent().get(&key).unwrap_or(0);
    env.storage().persistent().set(&key, &(current + amount));
}

pub fn get_voucher_balance(env: &Env, address: &Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::VoucherBalance(address.clone()))
        .unwrap_or(0)
}
