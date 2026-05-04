use soroban_sdk::{contracttype, Address, Bytes, BytesN, Symbol};

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

/// ABI/schema registry entry for a contract.
#[contracttype]
#[derive(Clone, Debug)]
pub struct ContractAbi {
    pub contract_id: BytesN<32>,
    pub version: u32,
    pub schema: Bytes,
}

#[contracttype]
pub enum DataKey {
    Escrow(BytesN<32>),
    Admin,
    Oracle,
    ApprovedVendor(BytesN<32>),
    /// Voucher balance per address
    VoucherBalance(Address),
    /// Approved vendor address → bool (for transfer restriction)
    VendorAddress(Address),
    /// ABI registry: contract_id → ContractAbi
    ContractAbi(BytesN<32>),
    /// ABI version counter per contract
    AbiVersion(BytesN<32>),
}

pub const APPROVAL_TIMEOUT_LEDGERS: u32 = 100_800;
pub const PROTOCOL_FEE_BPS: u32 = 100;
pub const DEFAULT_REPAYMENT_FEE_BPS: u32 = 1000;
