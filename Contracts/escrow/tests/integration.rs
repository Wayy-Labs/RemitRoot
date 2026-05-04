#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, BytesN, Env, Symbol,
};

use crate::{EscrowContract, EscrowContractClient};
use crate::storage::{EscrowState, REPAYMENT_WINDOW_LEDGERS};

fn setup(env: &Env) -> (Address, EscrowContractClient, Address, Address) {
    let contract_id = env.register_contract(None, EscrowContract);
    let client = EscrowContractClient::new(env, &contract_id);
    let admin = Address::generate(env);
    let oracle = Address::generate(env);
    client.initialize(&admin, &oracle);

    let usdc_admin = Address::generate(env);
    let usdc_token = env.register_stellar_asset_contract_v2(usdc_admin.clone());
    (usdc_token.address(), client, admin, oracle)
}

fn fund_and_redeem(
    env: &Env,
    client: &EscrowContractClient,
    usdc_id: &Address,
    amount: i128,
) -> (BytesN<32>, Address, Address) {
    let sender = Address::generate(env);
    let farmer = Address::generate(env);
    let vendor = Address::generate(env);
    let vendor_id: BytesN<32> = BytesN::from_array(env, &[1u8; 32]);

    let usdc_client = soroban_sdk::token::StellarAssetClient::new(env, usdc_id);
    usdc_client.mint(&sender, &amount);

    let escrow_id = client.fund(usdc_id, &sender, &vendor_id, &Symbol::new(env, "2025A"), &amount);
    client.approve_farmer(&escrow_id, &farmer);
    client.redeem_voucher(usdc_id, &escrow_id, &vendor);

    (escrow_id, farmer, sender)
}

#[test]
fn test_trigger_repay_sets_deadline() {
    let env = Env::default();
    env.mock_all_auths();

    let (usdc_id, client, _admin, _oracle) = setup(&env);
    let (escrow_id, _farmer, _sender) = fund_and_redeem(&env, &client, &usdc_id, 100_000_000);

    client.trigger_repay(&escrow_id);

    let record = client.get_escrow(&escrow_id);
    assert_eq!(record.state, EscrowState::Repaying);
    assert!(record.repay_deadline_ledger > 0);
}

#[test]
fn test_partial_repayment() {
    let env = Env::default();
    env.mock_all_auths();

    let (usdc_id, client, _admin, _oracle) = setup(&env);
    let amount: i128 = 100_000_000;
    let (escrow_id, farmer, _sender) = fund_and_redeem(&env, &client, &usdc_id, amount);

    client.trigger_repay(&escrow_id);

    // Mint repayment funds to farmer
    let usdc_client = soroban_sdk::token::StellarAssetClient::new(&env, &usdc_id);
    usdc_client.mint(&farmer, &200_000_000i128);

    // Partial repayment: 50 USDC
    client.repay(&usdc_id, &escrow_id, &farmer, &50_000_000i128);

    let record = client.get_escrow(&escrow_id);
    assert_eq!(record.repaid, 50_000_000i128);
    assert_eq!(record.state, EscrowState::Repaying); // not closed yet

    let remaining = client.get_remaining_balance(&escrow_id);
    // total_owed = 100M + 10% = 110M; remaining = 110M - 50M = 60M
    assert_eq!(remaining, 60_000_000i128);
}

#[test]
fn test_full_repayment_closes_escrow() {
    let env = Env::default();
    env.mock_all_auths();

    let (usdc_id, client, _admin, _oracle) = setup(&env);
    let amount: i128 = 100_000_000;
    let (escrow_id, farmer, _sender) = fund_and_redeem(&env, &client, &usdc_id, amount);

    client.trigger_repay(&escrow_id);

    let usdc_client = soroban_sdk::token::StellarAssetClient::new(&env, &usdc_id);
    // total_owed = 110M (principal + 10% fee)
    usdc_client.mint(&farmer, &110_000_000i128);

    client.repay(&usdc_id, &escrow_id, &farmer, &110_000_000i128);

    let record = client.get_escrow(&escrow_id);
    assert_eq!(record.state, EscrowState::Closed);
    assert_eq!(record.repaid, 110_000_000i128);
}

#[test]
fn test_overpayment_capped_at_remaining() {
    let env = Env::default();
    env.mock_all_auths();

    let (usdc_id, client, _admin, _oracle) = setup(&env);
    let amount: i128 = 100_000_000;
    let (escrow_id, farmer, _sender) = fund_and_redeem(&env, &client, &usdc_id, amount);

    client.trigger_repay(&escrow_id);

    let usdc_client = soroban_sdk::token::StellarAssetClient::new(&env, &usdc_id);
    // Mint more than owed
    usdc_client.mint(&farmer, &200_000_000i128);

    // Try to overpay — should be capped at 110M
    client.repay(&usdc_id, &escrow_id, &farmer, &200_000_000i128);

    let record = client.get_escrow(&escrow_id);
    assert_eq!(record.state, EscrowState::Closed);
    assert_eq!(record.repaid, 110_000_000i128);
}

#[test]
fn test_repayment_history_tracked() {
    let env = Env::default();
    env.mock_all_auths();

    let (usdc_id, client, _admin, _oracle) = setup(&env);
    let amount: i128 = 100_000_000;
    let (escrow_id, farmer, _sender) = fund_and_redeem(&env, &client, &usdc_id, amount);

    client.trigger_repay(&escrow_id);

    let usdc_client = soroban_sdk::token::StellarAssetClient::new(&env, &usdc_id);
    usdc_client.mint(&farmer, &110_000_000i128);

    client.repay(&usdc_id, &escrow_id, &farmer, &30_000_000i128);
    client.repay(&usdc_id, &escrow_id, &farmer, &40_000_000i128);
    client.repay(&usdc_id, &escrow_id, &farmer, &40_000_000i128);

    let history = client.get_repayment_history(&escrow_id);
    assert_eq!(history.len(), 3u32);
    assert_eq!(history.get(0).unwrap().amount, 30_000_000i128);
    assert_eq!(history.get(1).unwrap().amount, 40_000_000i128);
    // Last payment capped at remaining (110M - 70M = 40M)
    assert_eq!(history.get(2).unwrap().cumulative, 110_000_000i128);
}

#[test]
fn test_default_after_deadline() {
    let env = Env::default();
    env.mock_all_auths();

    let (usdc_id, client, _admin, _oracle) = setup(&env);
    let (escrow_id, _farmer, _sender) = fund_and_redeem(&env, &client, &usdc_id, 100_000_000);

    client.trigger_repay(&escrow_id);

    // Advance past repayment deadline
    env.ledger().with_mut(|l| {
        l.sequence_number += REPAYMENT_WINDOW_LEDGERS + 1;
    });

    client.default_escrow(&escrow_id);

    let record = client.get_escrow(&escrow_id);
    assert_eq!(record.state, EscrowState::Defaulted);
}

#[test]
#[should_panic]
fn test_repay_after_complete_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let (usdc_id, client, _admin, _oracle) = setup(&env);
    let amount: i128 = 100_000_000;
    let (escrow_id, farmer, _sender) = fund_and_redeem(&env, &client, &usdc_id, amount);

    client.trigger_repay(&escrow_id);

    let usdc_client = soroban_sdk::token::StellarAssetClient::new(&env, &usdc_id);
    usdc_client.mint(&farmer, &200_000_000i128);

    client.repay(&usdc_id, &escrow_id, &farmer, &110_000_000i128);

    // Should panic — already closed
    client.repay(&usdc_id, &escrow_id, &farmer, &1i128);
}
