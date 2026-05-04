use soroban_sdk::{contracttype, Address, BytesN, Symbol};

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
    pub state: EscrowState,
    pub created_ledger: u32,
}

#[contracttype]
pub enum DataKey {
    Escrow(BytesN<32>),
    Admin,
    Oracle,
    ApprovedVendor(BytesN<32>),
    VoucherBalance(Address),
}

pub const APPROVAL_TIMEOUT_LEDGERS: u32 = 100_800; // ~7 days at 6s/ledger
pub const PROTOCOL_FEE_BPS: u32 = 100; // 1%
pub const DEFAULT_REPAYMENT_FEE_BPS: u32 = 1000; // 10%
