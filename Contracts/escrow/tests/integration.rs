#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, BytesN, Env, Symbol,
};

use crate::{EscrowContract, EscrowContractClient};
use crate::storage::{EscrowState, APPROVAL_TIMEOUT_LEDGERS};

fn create_env() -> Env {
    Env::default()
}

fn register_contract(env: &Env) -> Address {
    env.register_contract(None, EscrowContract)
}

#[test]
fn test_fund_creates_escrow() {
    let env = create_env();
    env.mock_all_auths();

    let contract_id = register_contract(&env);
    let client = EscrowContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);
    client.initialize(&admin, &oracle);

    // Create a mock USDC token
    let usdc_admin = Address::generate(&env);
    let usdc_token = env.register_stellar_asset_contract_v2(usdc_admin.clone());
    let usdc_id = usdc_token.address();

    let sender = Address::generate(&env);
    let vendor_id: BytesN<32> = BytesN::from_array(&env, &[1u8; 32]);
    let crop_season = Symbol::new(&env, "2025A");
    let amount: i128 = 200_000_000; // 200 USDC (7 decimals)

    // Mint USDC to sender
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

    let contract_id = register_contract(&env);
    let client = EscrowContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);
    client.initialize(&admin, &oracle);

    let usdc_admin = Address::generate(&env);
    let usdc_token = env.register_stellar_asset_contract_v2(usdc_admin.clone());
    let usdc_id = usdc_token.address();

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

    let contract_id = register_contract(&env);
    let client = EscrowContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);
    client.initialize(&admin, &oracle);

    let usdc_admin = Address::generate(&env);
    let usdc_token = env.register_stellar_asset_contract_v2(usdc_admin.clone());
    let usdc_id = usdc_token.address();

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

    let contract_id = register_contract(&env);
    let client = EscrowContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);
    client.initialize(&admin, &oracle);

    let usdc_admin = Address::generate(&env);
    let usdc_token = env.register_stellar_asset_contract_v2(usdc_admin.clone());
    let usdc_id = usdc_token.address();

    let sender = Address::generate(&env);
    let vendor_id: BytesN<32> = BytesN::from_array(&env, &[4u8; 32]);
    let amount: i128 = 50_000_000;

    let usdc_client = soroban_sdk::token::StellarAssetClient::new(&env, &usdc_id);
    usdc_client.mint(&sender, &amount);

    let escrow_id = client.fund(&usdc_id, &sender, &vendor_id, &Symbol::new(&env, "2025D"), &amount);

    // Should panic — timeout not reached
    client.cancel(&usdc_id, &escrow_id);
}
