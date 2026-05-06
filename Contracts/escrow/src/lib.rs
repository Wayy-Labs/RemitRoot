#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Bytes, BytesN, Env, Symbol, Vec};

mod errors;
mod escrow;
mod events;
mod repayment;
mod storage;
mod voucher;

// Gas optimization modules
mod gas_profiler;
mod storage_optimized;
mod batch_ops;
mod gas_estimation;

use errors::Error;
use storage::{ContractAbi, DataKey, EscrowRecord, RepaymentEntry};

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

    /// Sender locks USDC for a specific farmer + vendor + season.
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

    /// Admin approves farmer and mints voucher.
    pub fn approve_farmer(
        env: Env,
        escrow_id: BytesN<32>,
        farmer: Address,
    ) -> Result<(), Error> {
        escrow::approve_farmer(&env, escrow_id, farmer)
    }

    /// Vendor burns voucher and receives USDC.
    pub fn redeem_voucher(
        env: Env,
        usdc_token: Address,
        escrow_id: BytesN<32>,
        vendor: Address,
    ) -> Result<(), Error> {
        escrow::redeem_voucher(&env, &usdc_token, escrow_id, vendor)
    }

    /// Oracle triggers repayment window after harvest.
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

    /// Cancel escrow and refund sender after timeout.
    pub fn cancel(env: Env, usdc_token: Address, escrow_id: BytesN<32>) -> Result<(), Error> {
        escrow::cancel(&env, &usdc_token, escrow_id)
    }

    /// Get escrow details.
    pub fn get_escrow(env: Env, escrow_id: BytesN<32>) -> Result<EscrowRecord, Error> {
        escrow::get_escrow(&env, escrow_id)
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
    // Voucher Token System
    // -----------------------------------------------------------------------

    /// Get RVCH voucher balance for an address.
    pub fn get_voucher_balance(env: Env, address: Address) -> i128 {
        voucher::get_voucher_balance(&env, &address)
    }

    // -----------------------------------------------------------------------
    // ABI / Schema Registry
    // -----------------------------------------------------------------------

    /// Register a new ABI/schema for a contract ID.
    pub fn register_abi(
        env: Env,
        contract_id: BytesN<32>,
        schema: Bytes,
    ) -> Result<u32, Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::Unauthorized)?;
        admin.require_auth();
        voucher::register_abi(&env, contract_id, schema)
    }

    /// Update an existing ABI/schema (increments version).
    pub fn update_abi(
        env: Env,
        contract_id: BytesN<32>,
        schema: Bytes,
    ) -> Result<u32, Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::Unauthorized)?;
        admin.require_auth();
        voucher::update_abi(&env, contract_id, schema)
    }

    /// Get the stored ABI/schema for a contract.
    pub fn get_abi(env: Env, contract_id: BytesN<32>) -> Result<ContractAbi, Error> {
        voucher::get_abi(&env, contract_id)
    }

    /// Validate that a schema matches the stored ABI.
    pub fn validate_abi(env: Env, contract_id: BytesN<32>, schema: Bytes) -> Result<bool, Error> {
        voucher::validate_abi(&env, contract_id, &schema)
    }

    // -----------------------------------------------------------------------
    // Admin
    // -----------------------------------------------------------------------

    /// Approve a vendor address for voucher redemption.
    pub fn approve_vendor_address(env: Env, vendor: Address) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::Unauthorized)?;
        admin.require_auth();
        env.storage()
            .persistent()
            .set(&DataKey::VendorAddress(vendor), &true);
        Ok(())
    }

    /// Remove a vendor address approval.
    pub fn remove_vendor_address(env: Env, vendor: Address) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::Unauthorized)?;
        admin.require_auth();
        env.storage()
            .persistent()
            .set(&DataKey::VendorAddress(vendor), &false);
        Ok(())
    }

    /// Approve a vendor by ID (BytesN<32>).
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

    /// Admin: set new oracle address.
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

    // -----------------------------------------------------------------------
    // Gas Optimization & Batch Operations
    // -----------------------------------------------------------------------

    /// Execute batch repayments for multiple escrows in a single transaction.
    /// Significantly reduces gas costs compared to individual repay calls.
    ///
    /// # Arguments
    /// * `usdc_token` - USDC token contract address
    /// * `items` - Vector of batch repayment items
    /// * `max_batch_size` - Maximum allowed batch size (default 50)
    ///
    /// # Returns
    /// BatchRepayResult with successful/failed counts and gas savings estimate
    pub fn batch_repay(
        env: Env,
        usdc_token: Address,
        items: Vec<batch_ops::BatchRepayItem>,
        max_batch_size: u32,
    ) -> Result<batch_ops::BatchRepayResult, Error> {
        batch_ops::batch_repay(&env, &usdc_token, items, max_batch_size)
    }

    /// Get gas cost estimate for a fund operation
    pub fn estimate_fund_cost(_env: Env) -> gas_estimation::FundEstimate {
        gas_estimation::FundEstimate::new()
    }

    /// Get gas cost estimate for an approve operation
    pub fn estimate_approve_cost(_env: Env) -> gas_estimation::ApproveEstimate {
        gas_estimation::ApproveEstimate::new()
    }

    /// Get gas cost estimate for a redeem operation with specified number of transfers
    pub fn estimate_redeem_cost(_env: Env, transfers: u32) -> gas_estimation::RedeemEstimate {
        gas_estimation::RedeemEstimate::new(transfers)
    }

    /// Get gas cost estimate for a repay operation
    pub fn estimate_repay_cost(_env: Env, transfers: u32, includes_history: bool) -> gas_estimation::RepayEstimate {
        gas_estimation::RepayEstimate::new(transfers, includes_history)
    }

    /// Get gas cost estimate for a batch repay operation
    pub fn estimate_batch_repay_cost(_env: Env, batch_size: u32) -> gas_estimation::BatchRepayEstimate {
        gas_estimation::BatchRepayEstimate::new(batch_size)
    }

    /// Get recent repayment entries (lazy loaded)
    /// Reduces gas by only fetching recent entries instead of full history
    pub fn get_recent_repayments(
        env: Env,
        escrow_id: BytesN<32>,
        limit: u32,
    ) -> Result<Vec<RepaymentEntry>, Error> {
        storage_optimized::get_recent_repayments(&env, &escrow_id, limit as usize)
    }

    /// Get repayment summary (total repaid and entry count) without loading full history
    pub fn get_repayment_summary(
        env: Env,
        escrow_id: BytesN<32>,
    ) -> Result<(i128, u32), Error> {
        storage_optimized::get_repayment_summary(&env, &escrow_id)
    }

    /// Prune old repayment history to maintain storage efficiency
    /// Admin-only operation
    pub fn prune_repayment_history(
        env: Env,
        escrow_id: BytesN<32>,
        max_entries: u32,
    ) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::Unauthorized)?;
        admin.require_auth();
        storage_optimized::prune_repayment_history(&env, &escrow_id, max_entries as usize)
    }

    /// Batch query multiple escrow records
    /// More efficient than multiple get_escrow calls
    pub fn batch_get_escrows(
        env: Env,
        escrow_ids: Vec<BytesN<32>>,
    ) -> Vec<Option<EscrowRecord>> {
        batch_ops::batch_get_escrows(&env, escrow_ids)
    }

    /// Get gas profiler constants for off-chain optimization
    pub fn get_gas_costs(_env: Env) -> (u32, u32, u32, u32) {
        (
            gas_profiler::gas_costs::STORAGE_READ_COST,
            gas_profiler::gas_costs::STORAGE_WRITE_COST,
            gas_profiler::gas_costs::TOKEN_TRANSFER_COST,
            gas_profiler::gas_costs::REPAY_BASE,
        )
    }
}