#![no_std]

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Symbol, Vec};

mod errors;
mod escrow;
mod events;
mod repayment;
mod storage;
mod voucher;

use errors::Error;
use storage::{DataKey, EscrowRecord, RepaymentEntry};

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Initialize the contract with admin and oracle addresses.
    pub fn initialize(env: Env, admin: Address, oracle: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Oracle, &oracle);
    }

    // -----------------------------------------------------------------------
    // Escrow lifecycle
    // -----------------------------------------------------------------------

    pub fn fund(
        env: Env,
        usdc_token: Address,
        sender: Address,
        vendor_id: BytesN<32>,
        crop_season: Symbol,
        amount: i128,
    ) -> Result<BytesN<32>, Error> {
        escrow::fund(&env, &usdc_token, &sender, vendor_id, crop_season, amount)
    }

    pub fn approve_farmer(env: Env, escrow_id: BytesN<32>, farmer: Address) -> Result<(), Error> {
        escrow::approve_farmer(&env, escrow_id, farmer)
    }

    pub fn redeem_voucher(
        env: Env,
        usdc_token: Address,
        escrow_id: BytesN<32>,
        vendor: Address,
    ) -> Result<(), Error> {
        escrow::redeem_voucher(&env, &usdc_token, escrow_id, vendor)
    }

    pub fn cancel(env: Env, usdc_token: Address, escrow_id: BytesN<32>) -> Result<(), Error> {
        escrow::cancel(&env, &usdc_token, escrow_id)
    }

    pub fn get_escrow(env: Env, escrow_id: BytesN<32>) -> Result<EscrowRecord, Error> {
        escrow::get_escrow(&env, escrow_id)
    }

    // -----------------------------------------------------------------------
    // Repayment Streaming Logic
    // -----------------------------------------------------------------------

    /// Oracle triggers the repayment window after harvest.
    pub fn trigger_repay(env: Env, escrow_id: BytesN<32>) -> Result<(), Error> {
        repayment::trigger_repay(&env, escrow_id)
    }

    /// Farmer makes a partial or full repayment.
    /// Automatically streams USDC back to sender.
    pub fn repay(
        env: Env,
        usdc_token: Address,
        escrow_id: BytesN<32>,
        farmer: Address,
        amount: i128,
    ) -> Result<(), Error> {
        repayment::repay(&env, &usdc_token, escrow_id, &farmer, amount)
    }

    /// Oracle triggers default on an overdue escrow.
    pub fn default_escrow(env: Env, escrow_id: BytesN<32>) -> Result<(), Error> {
        repayment::default_escrow(&env, escrow_id)
    }

    /// Get full repayment history for an escrow.
    pub fn get_repayment_history(env: Env, escrow_id: BytesN<32>) -> Vec<RepaymentEntry> {
        repayment::get_repayment_history(&env, escrow_id)
    }

    /// Get remaining balance to repay for an escrow.
    pub fn get_remaining_balance(env: Env, escrow_id: BytesN<32>) -> Result<i128, Error> {
        repayment::get_remaining_balance(&env, escrow_id)
    }

    // -----------------------------------------------------------------------
    // Voucher
    // -----------------------------------------------------------------------

    pub fn get_voucher_balance(env: Env, address: Address) -> i128 {
        voucher::get_voucher_balance(&env, &address)
    }

    // -----------------------------------------------------------------------
    // Admin
    // -----------------------------------------------------------------------

    pub fn approve_vendor(env: Env, vendor_id: BytesN<32>) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::Unauthorized)?;
        admin.require_auth();
        env.storage()
            .persistent()
            .set(&DataKey::ApprovedVendor(vendor_id), &true);
        Ok(())
    }

    pub fn set_oracle(env: Env, oracle: Address) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::Unauthorized)?;
        admin.require_auth();
        env.storage().instance().set(&DataKey::Oracle, &oracle);
        Ok(())
    }
}
