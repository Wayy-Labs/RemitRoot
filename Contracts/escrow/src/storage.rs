use soroban_sdk::{contracttype, Address, BytesN, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EscrowState {
    Created,
    Funded,
    VoucherMinted,
    Redeemed,
    Repaying,
    Closed,
    Defaulted,
}

/// Individual repayment event stored on-chain.
#[contracttype]
#[derive(Clone, Debug)]
pub struct RepaymentEntry {
    pub amount: i128,
    pub ledger: u32,
    pub cumulative: i128,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct EscrowRecord {
    pub sender: Address,
    pub farmer: Option<Address>,
    pub vendor_id: BytesN<32>,
    pub crop_season: Symbol,
    pub amount: i128,
    pub repaid: i128,
    pub repayment_fee_bps: u32,
    pub protocol_fee_bps: u32,
    pub state: EscrowState,
    pub created_ledger: u32,
    pub repay_deadline_ledger: u32,
}

#[contracttype]
pub enum DataKey {
    Escrow(BytesN<32>),
    Admin,
    Oracle,
    ApprovedVendor(BytesN<32>),
    VoucherBalance(Address),
    VendorAddress(Address),
    /// Repayment history list per escrow
    RepaymentHistory(BytesN<32>),
}

pub const APPROVAL_TIMEOUT_LEDGERS: u32 = 100_800; // ~7 days
pub const REPAYMENT_WINDOW_LEDGERS: u32 = 1_814_400; // ~126 days (harvest season)
pub const PROTOCOL_FEE_BPS: u32 = 100; // 1%
pub const DEFAULT_REPAYMENT_FEE_BPS: u32 = 1000; // 10%
