use soroban_sdk::{Address, BytesN, Env, Symbol, symbol_short};

pub fn escrow_funded(env: &Env, escrow_id: &BytesN<32>, sender: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("funded"), escrow_id.clone()),
        (sender.clone(), amount),
    );
}

pub fn farmer_approved(env: &Env, escrow_id: &BytesN<32>, farmer: &Address) {
    env.events().publish(
        (symbol_short!("approved"), escrow_id.clone()),
        farmer.clone(),
    );
}

pub fn voucher_minted(env: &Env, escrow_id: &BytesN<32>, farmer: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("minted"), escrow_id.clone()),
        (farmer.clone(), amount),
    );
}

pub fn voucher_burned(env: &Env, escrow_id: &BytesN<32>, vendor: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("burned"), escrow_id.clone()),
        (vendor.clone(), amount),
    );
}

pub fn voucher_redeemed(env: &Env, escrow_id: &BytesN<32>, vendor: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("redeemed"), escrow_id.clone()),
        (vendor.clone(), amount),
    );
}

pub fn repay_triggered(env: &Env, escrow_id: &BytesN<32>) {
    env.events().publish(
        (Symbol::new(env, "repay_trig"), escrow_id.clone()),
        (),
    );
}

pub fn repayment_made(env: &Env, escrow_id: &BytesN<32>, farmer: &Address, amount: i128, total_repaid: i128) {
    env.events().publish(
        (Symbol::new(env, "repayment"), escrow_id.clone()),
        (farmer.clone(), amount, total_repaid),
    );
}

pub fn escrow_closed(env: &Env, escrow_id: &BytesN<32>) {
    env.events().publish(
        (symbol_short!("closed"), escrow_id.clone()),
        (),
    );
}

pub fn escrow_cancelled(env: &Env, escrow_id: &BytesN<32>, sender: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("cancel"), escrow_id.clone()),
        (sender.clone(), amount),
    );
}

pub fn escrow_defaulted(env: &Env, escrow_id: &BytesN<32>) {
    env.events().publish(
        (symbol_short!("default"), escrow_id.clone()),
        (),
    );
}

pub fn abi_registered(env: &Env, contract_id: &BytesN<32>, version: u32) {
    env.events().publish(
        (Symbol::new(env, "abi_reg"), contract_id.clone()),
        version,
    );
}

pub fn abi_updated(env: &Env, contract_id: &BytesN<32>, old_version: u32, new_version: u32) {
    env.events().publish(
        (Symbol::new(env, "abi_upd"), contract_id.clone()),
        (old_version, new_version),
    );
}
