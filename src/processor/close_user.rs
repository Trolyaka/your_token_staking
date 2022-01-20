use crate::{
    error::CustomError,
    processor::create_user::get_user_storage_address_and_bump_seed,
    state::{
        AccTypesWithVersion, CwarPool, User, CWAR_POOL_STORAGE_TOTAL_BYTES,
        USER_STORAGE_TOTAL_BYTES,
    },
    utils,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use borsh::{BorshDeserialize, BorshSerialize};
/// 0. `[signer]` User Wallet Account
/// 1. `[writable]` User Storage Account
/// 2. `[writable]` CWAR Pool Storage Account
/// 3. `[writable]` CWAR Staking Vault
/// 4. `[writable]` CWAR Reward Vault
/// 5. `[writable]` CWAR ATA to Credit
/// 6. `[]` Pool Signer [pool storage, program id]
/// 7. `[]` Token Program
pub fn process_close_user(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user_wallet_account = next_account_info(account_info_iter)?;
    let user_storage_account = next_account_info(account_info_iter)?;
    let cwar_pool_storage_account = next_account_info(account_info_iter)?;

    if !user_wallet_account.is_signer {
        msg!("ProgramError::MissingRequiredSignature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (user_storage_address, _bump_seed) = get_user_storage_address_and_bump_seed(
        user_wallet_account.key,
        cwar_pool_storage_account.key,
        program_id,
    );
    if user_storage_address != *user_storage_account.key {
        msg!("Error: User Storage address does not match seed derivation");
        return Err(ProgramError::InvalidSeeds);
    }

    if cwar_pool_storage_account.data_len() != CWAR_POOL_STORAGE_TOTAL_BYTES {
        msg!("CustomError::DataSizeNotMatched");
        return Err(CustomError::DataSizeNotMatched.into());
    }
    let mut cwar_pool_data_byte_array = cwar_pool_storage_account.data.try_borrow_mut().unwrap();
    let mut cwar_pool_data: CwarPool =
        CwarPool::try_from_slice(&cwar_pool_data_byte_array[0usize..CWAR_POOL_STORAGE_TOTAL_BYTES])
            .unwrap();
    if cwar_pool_data.acc_type != AccTypesWithVersion::CwarPoolDataV1 as u8 {
        msg!("CustomError::ExpectedAccountTypeMismatched");
        return Err(CustomError::ExpectedAccountTypeMismatched.into());
    }

    cwar_pool_data.user_stake_count -= 1u32;

    cwar_pool_data_byte_array[0usize..CWAR_POOL_STORAGE_TOTAL_BYTES]
        .copy_from_slice(&cwar_pool_data.try_to_vec().unwrap());

    if user_storage_account.data_len() != USER_STORAGE_TOTAL_BYTES {
        msg!("CustomError::DataSizeNotMatched");
        return Err(CustomError::DataSizeNotMatched.into());
    }

    let mut user_data_byte_array = user_storage_account.data.try_borrow_mut().unwrap();
    let user_storage_data: User =
        User::try_from_slice(&user_data_byte_array[0usize..USER_STORAGE_TOTAL_BYTES]).unwrap();
    if user_storage_data.acc_type != AccTypesWithVersion::UserDataV1 as u8 {
        msg!("CustomError::ExpectedAccountTypeMismatched");
        return Err(CustomError::ExpectedAccountTypeMismatched.into());
    }

    if user_storage_data.user_wallet != *user_wallet_account.key {
        msg!("CustomError::UserStorageAuthorityMismatched");
        return Err(CustomError::UserStorageAuthorityMismatched.into());
    }
    if user_storage_data.cwar_pool != *cwar_pool_storage_account.key {
        msg!("CustomError::UserPoolMismatched");
        return Err(CustomError::UserPoolMismatched.into());
    }

    if user_storage_data.balance_cwar_staked != 0u64
        || user_storage_data.cwar_reward_per_token_pending != 0
    {
        msg!("CustomError::UserBalanceNonZero");
        return Err(CustomError::UserBalanceNonZero.into());
    }

    msg!("Closing the User Data Storage account and transferring lamports to User wallet...");
    utils::close_account(
        user_storage_account,
        user_wallet_account,
        &mut user_data_byte_array,
    )
    .unwrap();
    Ok(())
}
