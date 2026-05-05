#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Bytes, BytesN, Env, Symbol,
};

use crate::{EscrowContract, EscrowContractClient};
use crate::storage::{EscrowState, APPROVAL_TIMEOUT_LEDGERS};

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

fn create_env() -> Env {
    Env::default()
}

fn setup(env: &Env) -> (Address, EscrowContractClient, Address) {
    let contract_id = env.register_contract(None, EscrowContract);
    let client = EscrowContractClient::new(env, &contract_id);
    let admin = Address::generate(env);
    let oracle = Address::generate(env);
    client.initialize(&admin, &oracle);

    let usdc_admin = Address::generate(env);
    let usdc_token = env.register_stellar_asset_contract_v2(usdc_admin.clone());
    (usdc_token.address(), client, admin)
}

// ---------------------------------------------------------------------------
// Escrow lifecycle tests (from main)
// ---------------------------------------------------------------------------

#[test]
fn test_fund_creates_escrow() {
    let env = create_env();
    env.mock_all_auths();

    let (usdc_id, client, _admin) = setup(&env);

    let sender = Address::generate(&env);
    let vendor_id: BytesN<32> = BytesN::from_array(&env, &[1u8; 32]);
    let crop_season = Symbol::new(&env, "2025A");
    let amount: i128 = 200_000_000; // 200 USDC (7 decimals)

    let usdc_client = soroban_sdk::token::StellarAssetClient::new(&env, &usdc_id);
    usdc_client.mint(&sender, &amount);

    let escrow_id = client.fund(&usdc_id, &sender, &vendor_id, &crop_season, &amount);

    let record = client.get_escrow(&escrow_id);
    assert_eq!(record.sender, sender);
    assert_eq!(record.amount, amount);
    assert_eq!(record.state, EscrowState::Funded);
}

#[test]
fn test_approve_farmer_transitions_state() {
    let env = create_env();
    env.mock_all_auths();

    let (usdc_id, client, _admin) = setup(&env);

    let sender = Address::generate(&env);
    let farmer = Address::generate(&env);
    let vendor_id: BytesN<32> = BytesN::from_array(&env, &[2u8; 32]);
    let amount: i128 = 100_000_000;

    let usdc_client = soroban_sdk::token::StellarAssetClient::new(&env, &usdc_id);
    usdc_client.mint(&sender, &amount);

    let escrow_id = client.fund(&usdc_id, &sender, &vendor_id, &Symbol::new(&env, "2025B"), &amount);
    client.approve_farmer(&escrow_id, &farmer);

    let record = client.get_escrow(&escrow_id);
    assert_eq!(record.state, EscrowState::VoucherMinted);
    assert_eq!(record.farmer, Some(farmer));
}

#[test]
fn test_cancel_after_timeout() {
    let env = create_env();
    env.mock_all_auths();

    let (usdc_id, client, _admin) = setup(&env);

    let sender = Address::generate(&env);
    let vendor_id: BytesN<32> = BytesN::from_array(&env, &[3u8; 32]);
    let amount: i128 = 50_000_000;

    let usdc_client = soroban_sdk::token::StellarAssetClient::new(&env, &usdc_id);
    usdc_client.mint(&sender, &amount);

    let escrow_id = client.fund(&usdc_id, &sender, &vendor_id, &Symbol::new(&env, "2025C"), &amount);

    // Advance ledger past timeout
    env.ledger().with_mut(|l| {
        l.sequence_number += APPROVAL_TIMEOUT_LEDGERS + 1;
    });

    client.cancel(&usdc_id, &escrow_id);

    let record = client.get_escrow(&escrow_id);
    assert_eq!(record.state, EscrowState::Closed);
}

#[test]
#[should_panic]
fn test_cancel_before_timeout_fails() {
    let env = create_env();
    env.mock_all_auths();

    let (usdc_id, client, _admin) = setup(&env);

    let sender = Address::generate(&env);
    let vendor_id: BytesN<32> = BytesN::from_array(&env, &[4u8; 32]);
    let amount: i128 = 50_000_000;

    let usdc_client = soroban_sdk::token::StellarAssetClient::new(&env, &usdc_id);
    usdc_client.mint(&sender, &amount);

    let escrow_id = client.fund(&usdc_id, &sender, &vendor_id, &Symbol::new(&env, "2025D"), &amount);

    // Should panic — timeout not reached
    client.cancel(&usdc_id, &escrow_id);
}

// ---------------------------------------------------------------------------
// Voucher token system tests (from feature branch)
// ---------------------------------------------------------------------------

#[test]
fn test_mint_voucher_on_approve_farmer() {
    let env = create_env();
    env.mock_all_auths();

    let (usdc_id, client, _admin) = setup(&env);

    let sender = Address::generate(&env);
    let farmer = Address::generate(&env);
    let vendor = Address::generate(&env);
    let vendor_id: BytesN<32> = BytesN::from_array(&env, &[1u8; 32]);

    let usdc_client = soroban_sdk::token::StellarAssetClient::new(&env, &usdc_id);
    usdc_client.mint(&sender, &200_000_000i128);

    // Approve vendor address for transfer restriction
    client.approve_vendor_address(&vendor);

    let escrow_id = client.fund(&usdc_id, &sender, &vendor_id, &Symbol::new(&env, "2025A"), &200_000_000i128);

    // Before approval: balance is 0
    assert_eq!(client.get_voucher_balance(&farmer), 0);

    client.approve_farmer(&escrow_id, &farmer);

    // After approval: farmer has voucher balance
    assert_eq!(client.get_voucher_balance(&farmer), 200_000_000i128);
}

#[test]
fn test_burn_voucher_on_redeem() {
    let env = create_env();
    env.mock_all_auths();

    let (usdc_id, client, _admin) = setup(&env);

    let sender = Address::generate(&env);
    let farmer = Address::generate(&env);
    let vendor = Address::generate(&env);
    let vendor_id: BytesN<32> = BytesN::from_array(&env, &[2u8; 32]);
    let amount: i128 = 100_000_000;

    let usdc_client = soroban_sdk::token::StellarAssetClient::new(&env, &usdc_id);
    usdc_client.mint(&sender, &amount);

    client.approve_vendor_address(&vendor);

    let escrow_id = client.fund(&usdc_id, &sender, &vendor_id, &Symbol::new(&env, "2025B"), &amount);
    client.approve_farmer(&escrow_id, &farmer);

    assert_eq!(client.get_voucher_balance(&farmer), amount);

    client.redeem_voucher(&usdc_id, &escrow_id, &vendor);

    // After redemption: voucher balance burned
    assert_eq!(client.get_voucher_balance(&farmer), 0);
}

#[test]
fn test_abi_registry_register_and_get() {
    let env = create_env();
    env.mock_all_auths();

    let (_, client, _admin) = setup(&env);

    let contract_id: BytesN<32> = BytesN::from_array(&env, &[10u8; 32]);
    let schema = Bytes::from_slice(&env, b"{\"type\":\"escrow\",\"version\":1}");

    let version = client.register_abi(&contract_id, &schema);
    assert_eq!(version, 1u32);

    let abi = client.get_abi(&contract_id);
    assert_eq!(abi.version, 1u32);
    assert_eq!(abi.schema, schema);
}

#[test]
fn test_abi_registry_update_increments_version() {
    let env = create_env();
    env.mock_all_auths();

    let (_, client, _admin) = setup(&env);

    let contract_id: BytesN<32> = BytesN::from_array(&env, &[11u8; 32]);
    let schema_v1 = Bytes::from_slice(&env, b"{\"version\":1}");
    let schema_v2 = Bytes::from_slice(&env, b"{\"version\":2}");

    client.register_abi(&contract_id, &schema_v1);
    let new_version = client.update_abi(&contract_id, &schema_v2);
    assert_eq!(new_version, 2u32);

    let abi = client.get_abi(&contract_id);
    assert_eq!(abi.version, 2u32);
    assert_eq!(abi.schema, schema_v2);
}

#[test]
fn test_abi_validate_matching_schema() {
    let env = create_env();
    env.mock_all_auths();

    let (_, client, _admin) = setup(&env);

    let contract_id: BytesN<32> = BytesN::from_array(&env, &[12u8; 32]);
    let schema = Bytes::from_slice(&env, b"{\"type\":\"voucher\"}");

    client.register_abi(&contract_id, &schema);

    assert!(client.validate_abi(&contract_id, &schema));
    let wrong = Bytes::from_slice(&env, b"{\"type\":\"other\"}");
    assert!(!client.validate_abi(&contract_id, &wrong));
}

#[test]
#[should_panic]
fn test_transfer_to_unapproved_vendor_fails() {
    let env = create_env();
    env.mock_all_auths();

    let (usdc_id, client, _admin) = setup(&env);

    let sender = Address::generate(&env);
    let farmer = Address::generate(&env);
    let unapproved_vendor = Address::generate(&env); // NOT approved
    let vendor_id: BytesN<32> = BytesN::from_array(&env, &[3u8; 32]);
    let amount: i128 = 50_000_000;

    let usdc_client = soroban_sdk::token::StellarAssetClient::new(&env, &usdc_id);
    usdc_client.mint(&sender, &amount);

    let escrow_id = client.fund(&usdc_id, &sender, &vendor_id, &Symbol::new(&env, "2025C"), &amount);
    client.approve_farmer(&escrow_id, &farmer);

    // Should panic — vendor not approved
    client.redeem_voucher(&usdc_id, &escrow_id, &unapproved_vendor);
}