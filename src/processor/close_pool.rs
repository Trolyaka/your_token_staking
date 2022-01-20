use crate::{
    error::CustomError,
    state::{AccTypesWithVersion, CwarPool, CWAR_POOL_STORAGE_TOTAL_BYTES},
};
use solana_program::sysvar::clock::Clock;
use solana_program::sysvar::Sysvar;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::state::Account as TokenAccount;

/// 0. `[signer]` Pool Owner Wallet Account
/// 1. `[writable]` CWAR Staking Vault
/// 2. `[writable]` CWAR Staking Refund ATA
/// 3. `[writable]` CWAR Rewards Vault
/// 4. `[writable]` CWAR Rewards Refund ATA
/// 5. `[writable]` CWAR Pool Storage Account
/// 6. `[]` Token Program
pub fn process_close_pool(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pool_owner_wallet_account = next_account_info(account_info_iter)?;
    let cwar_staking_vault = next_account_info(account_info_iter)?;
    let cwar_staking_refund_ata = next_account_info(account_info_iter)?;
    let cwar_rewards_vault = next_account_info(account_info_iter)?;
    let cwar_rewards_refund_ata = next_account_info(account_info_iter)?;
    let cwar_pool_storage_account = next_account_info(account_info_iter)?;
    let pool_signer_pda = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

    if !pool_owner_wallet_account.is_signer {
        msg!("ProgramError::MissingRequiredSignature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    if token_program.key != &spl_token::id() {
        msg!("CustomError::InvalidTokenProgram");
        return Err(CustomError::InvalidTokenProgram.into());
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

    if cwar_pool_data.owner_wallet != *pool_owner_wallet_account.key {
        msg!("CustomError::PoolOwnerMismatched");
        return Err(CustomError::PoolOwnerMismatched.into());
    }

    if cwar_staking_vault.owner != token_program.key {
        msg!("CustomError::AccountOwnerShouldBeTokenProgram");
        return Err(CustomError::AccountOwnerShouldBeTokenProgram.into());
    }

    let cwar_staking_vault_data = TokenAccount::unpack(&cwar_staking_vault.data.borrow())?;
    let (pool_signer_address, bump_seed) =
        Pubkey::find_program_address(&[&cwar_pool_storage_account.key.to_bytes()], program_id);

    if cwar_staking_vault_data.owner != pool_signer_address
        || cwar_pool_data.cwar_staking_vault != *cwar_staking_vault.key
    {
        msg!("CustomError::InvalidStakingVault");
        return Err(CustomError::InvalidStakingVault.into());
    }

    let total_cwar_staked = cwar_staking_vault_data.amount;

    let now = Clock::get()?.unix_timestamp;
    if cwar_pool_data.reward_duration_end <= 0u64
        || cwar_pool_data.reward_duration_end >= (now as u64)
        || cwar_pool_data.user_stake_count != 0u32
        || total_cwar_staked != 0u64
    {
        msg!("CustomError::PoolStillActive");
        return Err(CustomError::PoolStillActive.into());
    }

    msg!("Calling the token program to transfer CWAR to Staking Refundee from Staking Vault...");
    invoke_signed(
        &spl_token::instruction::transfer(
            token_program.key,
            cwar_staking_vault.key,
            cwar_staking_refund_ata.key,
            &pool_signer_address,
            &[&pool_signer_address],
            cwar_staking_vault_data.amount,
        )?,
        &[
            cwar_staking_vault.clone(),
            cwar_staking_refund_ata.clone(),
            pool_signer_pda.clone(),
            token_program.clone(),
        ],
        &[&[&cwar_pool_storage_account.key.to_bytes()[..], &[bump_seed]]],
    )?;

    let cwar_rewards_vault_data = TokenAccount::unpack(&cwar_rewards_vault.data.borrow())?;
    let (pool_signer_address, bump_seed) =
        Pubkey::find_program_address(&[&cwar_pool_storage_account.key.to_bytes()], program_id);

    if cwar_rewards_vault_data.owner != pool_signer_address
        || cwar_pool_data.cwar_reward_vault != *cwar_rewards_vault.key
    {
        msg!("CustomError::InvalidRewardsVault");
        return Err(CustomError::InvalidRewardsVault.into());
    }

    msg!("Calling the token program to transfer CWAR to Rewards Refundee from Rewards Vault...");
    invoke_signed(
        &spl_token::instruction::transfer(
            token_program.key,
            cwar_rewards_vault.key,
            cwar_rewards_refund_ata.key,
            &pool_signer_address,
            &[&pool_signer_address],
            cwar_rewards_vault_data.amount,
        )?,
        &[
            cwar_rewards_vault.clone(),
            cwar_rewards_refund_ata.clone(),
            pool_signer_pda.clone(),
            token_program.clone(),
        ],
        &[&[&cwar_pool_storage_account.key.to_bytes()[..], &[bump_seed]]],
    )?;

    msg!("Calling the token program to close CWAR Staking Vault...");
    invoke_signed(
        &spl_token::instruction::close_account(
            token_program.key,
            cwar_staking_vault.key,
            pool_owner_wallet_account.key,
            &pool_signer_address,
            &[&pool_signer_address],
        )?,
        &[
            cwar_staking_vault.clone(),
            pool_owner_wallet_account.clone(),
            pool_signer_pda.clone(),
            token_program.clone(),
        ],
        &[&[&cwar_pool_storage_account.key.to_bytes()[..], &[bump_seed]]],
    )?;

    msg!("Calling the token program to close CWAR Rewards Vault...");
    invoke_signed(
        &spl_token::instruction::close_account(
            token_program.key,
            cwar_rewards_vault.key,
            pool_owner_wallet_account.key,
            &pool_signer_address,
            &[&pool_signer_address],
        )?,
        &[
            cwar_rewards_vault.clone(),
            pool_owner_wallet_account.clone(),
            pool_signer_pda.clone(),
            token_program.clone(),
        ],
        &[&[&cwar_pool_storage_account.key.to_bytes()[..], &[bump_seed]]],
    )?;

    cwar_pool_data.cwar_staking_vault = Pubkey::default();
    cwar_pool_data.cwar_reward_vault = Pubkey::default();
    cwar_pool_data_byte_array[0usize..CWAR_POOL_STORAGE_TOTAL_BYTES]
        .copy_from_slice(&cwar_pool_data.try_to_vec().unwrap());
    Ok(())
}
