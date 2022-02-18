use crate::{
    error::CustomError,
    state::{YourPool, YOUR_POOL_STORAGE_TOTAL_BYTES},
};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process_update_rates(
    accounts: &[AccountInfo],
    _program_id: &Pubkey,
    rewards_per_slot: u64,
    max_reward_rate: u64,
    min_reward_rate: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pool_owner_wallet_account = next_account_info(account_info_iter)?;
    let your_pool_storage_account = next_account_info(account_info_iter)?;

    if !pool_owner_wallet_account.is_signer {
        msg!("ProgramError::MissingRequiredSignature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut your_pool_data_byte_array = your_pool_storage_account.data.try_borrow_mut().unwrap();
    let mut your_pool_data: YourPool =
        YourPool::try_from_slice(&your_pool_data_byte_array[0usize..YOUR_POOL_STORAGE_TOTAL_BYTES])
            .unwrap();

    if your_pool_data.owner_wallet != *pool_owner_wallet_account.key {
        msg!("CustomError::PoolOwnerMismatched");
        return Err(CustomError::PoolOwnerMismatched.into());
    }

    your_pool_data.rewards_per_slot = rewards_per_slot;
    your_pool_data.max_reward_rate = max_reward_rate;
    your_pool_data.min_reward_rate = min_reward_rate;

    your_pool_data_byte_array[0usize..YOUR_POOL_STORAGE_TOTAL_BYTES]
        .copy_from_slice(&your_pool_data.try_to_vec().unwrap());

    Ok(())
}
