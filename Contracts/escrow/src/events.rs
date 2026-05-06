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

/// Emitted by voucher::burn_voucher — low-level burn inside the voucher module.
pub fn voucher_burned(env: &Env, escrow_id: &BytesN<32>, vendor: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("burned"), escrow_id.clone()),
        (vendor.clone(), amount),
    );
}

/// Emitted by escrow::redeem_voucher — high-level redemption with USDC transfer.
pub fn voucher_redeemed(env: &Env, escrow_id: &BytesN<32>, vendor: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("redeemed"), escrow_id.clone()),
        (vendor.clone(), amount),
    );
}

/// Emitted when oracle opens the repayment window. Includes deadline for off-chain monitoring.
pub fn repay_triggered(env: &Env, escrow_id: &BytesN<32>, deadline_ledger: u32) {
    env.events().publish(
        (Symbol::new(env, "repay_trig"), escrow_id.clone()),
        deadline_ledger,
    );
}

pub fn repayment_made(
    env: &Env,
    escrow_id: &BytesN<32>,
    farmer: &Address,
    amount: i128,
    total_repaid: i128,
    remaining: i128,
) {
    env.events().publish(
        (Symbol::new(env, "repayment"), escrow_id.clone()),
        (farmer.clone(), amount, total_repaid, remaining),
    );
}

pub fn repayment_complete(env: &Env, escrow_id: &BytesN<32>, total_repaid: i128) {
    env.events().publish(
        (Symbol::new(env, "repay_done"), escrow_id.clone()),
        total_repaid,
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

/// Emitted on default — includes repaid vs owed for off-chain loss accounting.
pub fn escrow_defaulted(env: &Env, escrow_id: &BytesN<32>, repaid: i128, owed: i128) {
    env.events().publish(
        (symbol_short!("default"), escrow_id.clone()),
        (repaid, owed),
    );
}

pub fn fee_collected(env: &Env, escrow_id: &BytesN<32>, protocol_fee: i128, repayment_fee: i128) {
    env.events().publish(
        (Symbol::new(env, "fee_coll"), escrow_id.clone()),
        (protocol_fee, repayment_fee),
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

/// Emitted when batch repay operation completes
pub fn batch_repay_completed(env: &Env, successful: u32, failed: u32, total_repaid: i128) {
    env.events().publish(
        (Symbol::new(env, "batch_repay"), "completed"),
        (successful, failed, total_repaid),
    );
}