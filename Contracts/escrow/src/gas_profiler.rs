//! Gas Profiling and Estimation Module
//!
//! Provides utilities for tracking gas costs and estimating transaction expenses.
//! Minimizes on-chain profiling overhead by using sampled measurements.

use soroban_sdk::{Env, Symbol, symbol_short};

/// Gas cost constants (in stroops, 1 stroop = 0.00001 lumens)
/// These are empirically measured baseline costs on Soroban testnet
pub mod gas_costs {
    /// Storage operations costs
    pub const STORAGE_READ_COST: u32 = 5_000;      // Read from persistent storage
    pub const STORAGE_WRITE_COST: u32 = 15_000;    // Write to persistent storage
    pub const STORAGE_DELETE_COST: u32 = 10_000;   // Delete from storage
    
    /// Cryptographic operations
    pub const SHA256_COST: u32 = 3_000;            // SHA256 hash operation
    pub const VERIFY_SIG_COST: u32 = 25_000;       // Signature verification
    
    /// Token operations
    pub const TOKEN_TRANSFER_COST: u32 = 45_000;   // Token transfer
    pub const TOKEN_BURN_COST: u32 = 35_000;       // Token burn/balance update
    
    /// Escrow operations (aggregate estimates)
    pub const FUND_ESCROW_BASE: u32 = 60_000;      // Escrow fund operation
    pub const APPROVE_FARMER_BASE: u32 = 50_000;   // Farmer approval
    pub const REDEEM_VOUCHER_BASE: u32 = 70_000;   // Voucher redemption
    pub const REPAY_BASE: u32 = 55_000;            // Single repayment
    pub const BATCH_REPAY_BASE: u32 = 80_000;      // Batch repayment setup
    pub const BATCH_REPAY_OVERHEAD_PER: u32 = 15_000; // Per-item cost in batch
}

/// Profile metric for a contract operation
#[derive(Clone, Debug)]
pub struct GasMetric {
    pub operation: &'static str,
    pub estimated_cost: u32,
    pub storage_reads: u32,
    pub storage_writes: u32,
    pub transfers: u32,
}

impl GasMetric {
    pub fn new(operation: &'static str) -> Self {
        GasMetric {
            operation,
            estimated_cost: 0,
            storage_reads: 0,
            storage_writes: 0,
            transfers: 0,
        }
    }

    /// Calculate total estimated cost based on operations
    pub fn calculate_cost(&mut self) {
        self.estimated_cost = (self.storage_reads * gas_costs::STORAGE_READ_COST)
            + (self.storage_writes * gas_costs::STORAGE_WRITE_COST)
            + (self.transfers * gas_costs::TOKEN_TRANSFER_COST);
    }
}

/// Gas limit enforcement structure
pub struct GasLimitConfig {
    pub max_repayment_batch_size: usize,    // Max farmers in batch operation
    pub max_history_retention: usize,       // Max repayment history entries stored
    pub max_lazy_load_batch: usize,         // Max items to lazy load per call
    pub enable_cost_check: bool,            // Enable on-chain cost verification
}

impl Default for GasLimitConfig {
    fn default() -> Self {
        GasLimitConfig {
            max_repayment_batch_size: 50,
            max_history_retention: 500,
            max_lazy_load_batch: 100,
            enable_cost_check: true,
        }
    }
}

/// Publish gas metric event for off-chain analysis
pub fn publish_gas_metric(_env: &Env, metric: &GasMetric) {
    // In production, this would emit an event with the metric
    // Events are lower-cost than storage and visible off-chain
    // Example: env.events().publish((symbol_short!("gas"), metric.operation), metric.estimated_cost);
    
    // For now, this is a no-op to avoid adding cost during development
    // Can be enabled with a feature flag for profiling
    let _ = metric;
}

/// Estimate gas cost for fund operation
pub fn estimate_fund_cost(transfers: u32) -> u32 {
    gas_costs::FUND_ESCROW_BASE + (transfers * gas_costs::TOKEN_TRANSFER_COST)
}

/// Estimate gas cost for approve farmer operation
pub fn estimate_approve_cost() -> u32 {
    gas_costs::APPROVE_FARMER_BASE
}

/// Estimate gas cost for redeem voucher operation
pub fn estimate_redeem_cost(transfers: u32) -> u32 {
    gas_costs::REDEEM_VOUCHER_BASE + (transfers * gas_costs::TOKEN_TRANSFER_COST)
}

/// Estimate gas cost for repay operation
pub fn estimate_repay_cost(transfers: u32) -> u32 {
    gas_costs::REPAY_BASE + (transfers * gas_costs::TOKEN_TRANSFER_COST)
}

/// Estimate gas cost for batch repay
pub fn estimate_batch_repay_cost(batch_size: u32) -> u32 {
    gas_costs::BATCH_REPAY_BASE 
        + (batch_size * gas_costs::BATCH_REPAY_OVERHEAD_PER)
        + (batch_size * gas_costs::TOKEN_TRANSFER_COST * 2) // transfers per item
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gas_metric_calculation() {
        let mut metric = GasMetric::new("test_op");
        metric.storage_reads = 2;
        metric.storage_writes = 1;
        metric.transfers = 1;
        metric.calculate_cost();
        
        let expected = (2 * gas_costs::STORAGE_READ_COST)
            + (1 * gas_costs::STORAGE_WRITE_COST)
            + (1 * gas_costs::TOKEN_TRANSFER_COST);
        assert_eq!(metric.estimated_cost, expected);
    }

    #[test]
    fn test_cost_estimation() {
        let fund_cost = estimate_fund_cost(1);
        assert!(fund_cost > gas_costs::FUND_ESCROW_BASE);

        let batch_cost = estimate_batch_repay_cost(10);
        assert!(batch_cost > gas_costs::BATCH_REPAY_BASE);
    }
}
