#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Bytes, BytesN, Env, Symbol};

mod errors;
mod escrow;
mod events;
mod repayment;
mod storage;
mod voucher;

use errors::Error;
use storage::{ContractAbi, DataKey, EscrowRecord};

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

    pub fn trigger_repay(env: Env, escrow_id: BytesN<32>) -> Result<(), Error> {
        escrow::trigger_repay(&env, escrow_id)
    }

    pub fn repay(
        env: Env,
        usdc_token: Address,
        escrow_id: BytesN<32>,
        farmer: Address,
        amount: i128,
    ) -> Result<(), Error> {
        repayment::repay(&env, &usdc_token, escrow_id, &farmer, amount)
    }

    pub fn default_escrow(env: Env, escrow_id: BytesN<32>) -> Result<(), Error> {
        repayment::default_escrow(&env, escrow_id)
    }

    pub fn cancel(env: Env, usdc_token: Address, escrow_id: BytesN<32>) -> Result<(), Error> {
        escrow::cancel(&env, &usdc_token, escrow_id)
    }

    pub fn get_escrow(env: Env, escrow_id: BytesN<32>) -> Result<EscrowRecord, Error> {
        escrow::get_escrow(&env, escrow_id)
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

    /// Set oracle address.
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
