use std::cell::RefMut;
use std::convert::TryInto;

use crate::error::CustomError;
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;

// to avoid rounding errors
const PRECISION: u128 = u64::MAX as u128;

pub mod constants {
    pub const MIN_DURATION: u64 = 86400; // 1 day
}

pub fn close_account(
    account_to_close: &AccountInfo,
    sol_receiving_account: &AccountInfo,
    account_to_close_data_byte_array: &mut RefMut<&mut [u8]>,
) -> Result<(), CustomError> {
    **sol_receiving_account.lamports.borrow_mut() = sol_receiving_account
        .lamports()
        .checked_add(account_to_close.lamports())
        .ok_or(CustomError::AmountOverflow)?;
    **account_to_close.lamports.borrow_mut() = 0;
    **account_to_close_data_byte_array = &mut [];
    Ok(())
}

pub fn rewards_per_token(
    total_your_staked: u64,
    last_time_reward_applicable: u64,
    total_stake_last_update_time: u64,
    your_reward_rate: u64,
    your_reward_per_token_stored: u128,
) -> Result<u128, ProgramError> {
    if total_your_staked == 0 {
        return Ok(your_reward_per_token_stored);
    }
    let new_reward_per_token_stored: u128 = (your_reward_rate as u128)
        .checked_mul(
            (last_time_reward_applicable as u128)
                .checked_sub(total_stake_last_update_time as u128)
                .ok_or(CustomError::AmountOverflow)?,
        )
        .ok_or(CustomError::AmountOverflow)?;
    let new_reward_per_token_stored_with_precision: u128 = new_reward_per_token_stored
        .checked_mul(PRECISION)
        .ok_or(CustomError::AmountOverflow)?;
    let updated_rewards_per_token_stored = your_reward_per_token_stored
        .checked_add(
            new_reward_per_token_stored_with_precision
                .checked_div(total_your_staked as u128)
                .ok_or(CustomError::AmountOverflow)?,
        )
        .ok_or(CustomError::AmountOverflow)?;
    return Ok(updated_rewards_per_token_stored);
}

pub fn earned(
    balance_your_staked: u64,
    reward_per_token_stored: u128,
    reward_per_token_complete: u128,
    reward_per_token_pending: u64,
) -> Result<u64, ProgramError> {
    let diff_reward_per_token = reward_per_token_stored
        .checked_sub(reward_per_token_complete)
        .ok_or(CustomError::AmountOverflow)?;
    let mul = ((balance_your_staked as u128)
        .checked_mul(diff_reward_per_token)
        .ok_or(CustomError::AmountOverflow)?)
    .checked_div(PRECISION)
    .ok_or(CustomError::AmountOverflow)? as u64;
    let updated_reward_per_token_pending = reward_per_token_pending
        .checked_add(mul)
        .ok_or(CustomError::AmountOverflow)?;
    return Ok(updated_reward_per_token_pending);
}

pub fn last_time_reward_applicable(reward_duration_end: u64, now_unix_timestamp: i64) -> u64 {
    return std::cmp::min(now_unix_timestamp.try_into().unwrap(), reward_duration_end);
}

