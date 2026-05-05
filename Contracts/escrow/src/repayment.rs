use soroban_sdk::{Address, BytesN, Env, token};

use crate::errors::Error;
use crate::events;
use crate::storage::{DataKey, EscrowRecord, EscrowState};

/// Farmer makes a partial or full repayment.
pub fn repay(
    env: &Env,
    usdc_token: &Address,
    escrow_id: BytesN<32>,
    farmer: &Address,
    amount: i128,
) -> Result<(), Error> {
    if amount <= 0 {
        return Err(Error::InvalidAmount);
    }

    farmer.require_auth();

    let mut record: EscrowRecord = env
        .storage()
        .persistent()
        .get(&DataKey::Escrow(escrow_id.clone()))
        .ok_or(Error::EscrowNotFound)?;

    if record.state != EscrowState::Repaying {
        return Err(Error::InvalidState);
    }

    // Total owed = principal + repayment fee
    let fee = record.amount * (record.repayment_fee_bps as i128) / 10_000;
    let total_owed = record.amount + fee;
    let remaining = total_owed - record.repaid;

    if remaining <= 0 {
        return Err(Error::RepaymentComplete);
    }

    // Cap at remaining balance
    let actual_amount = if amount > remaining { remaining } else { amount };

    let token_client = token::Client::new(env, usdc_token);
    token_client.transfer(farmer, &env.current_contract_address(), &actual_amount);

    // Stream repayment to sender
    token_client.transfer(&env.current_contract_address(), &record.sender, &actual_amount);

    record.repaid += actual_amount;

    if record.repaid >= total_owed {
        record.state = EscrowState::Closed;
        events::escrow_closed(env, &escrow_id);
    }

    events::repayment_made(env, &escrow_id, farmer, actual_amount, record.repaid);
    env.storage().persistent().set(&DataKey::Escrow(escrow_id), &record);

    Ok(())
}

/// Oracle-triggered default — marks escrow as defaulted.
pub fn default_escrow(env: &Env, escrow_id: BytesN<32>) -> Result<(), Error> {
    let oracle: Address = env.storage().instance().get(&DataKey::Oracle).ok_or(Error::Unauthorized)?;
    oracle.require_auth();

    let mut record: EscrowRecord = env
        .storage()
        .persistent()
        .get(&DataKey::Escrow(escrow_id.clone()))
        .ok_or(Error::EscrowNotFound)?;

    if record.state != EscrowState::Repaying {
        return Err(Error::InvalidState);
    }

    record.state = EscrowState::Defaulted;
    env.storage().persistent().set(&DataKey::Escrow(escrow_id.clone()), &record);
    events::escrow_defaulted(env, &escrow_id);

    Ok(())
}
