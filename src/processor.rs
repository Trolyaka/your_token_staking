use crate::instruction::Instruction;

use {
    claim_rewards::process_claim_rewards, create_user::process_create_user,
    initialize_cwar_pool::process_initialize_cwar_pool, stake_cwar::process_stake_cwar,
    unstake_cwar::process_unstake_cwar, close_pool::process_close_pool,
    close_user::process_close_user
};

pub mod close_pool;
pub mod close_user;
pub mod claim_rewards;
pub mod create_user;
pub mod initialize_cwar_pool;
pub mod stake_cwar;
pub mod unstake_cwar;

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = Instruction::unpack(instruction_data)?;
        match instruction {
            Instruction::InitializePool {
                reward_duration,
                pool_nonce,
            } => {
                msg!("Instruction::InitializePool");
                process_initialize_cwar_pool(accounts, reward_duration, pool_nonce, program_id)
            }
            Instruction::CreateUser { nonce } => {
                msg!("Instruction::CreateUser");
                process_create_user(accounts, nonce, program_id)
            }

            Instruction::Stake { amount_to_deposit } => {
                msg!("Instruction::Stake");
                process_stake_cwar(accounts, amount_to_deposit, program_id)
            }

            Instruction::Unstake { amount_to_withdraw } => {
                msg!("Instruction::Unstake");
                process_unstake_cwar(accounts, amount_to_withdraw, program_id)
            }

            Instruction::ClaimRewards {} => {
                msg!("Instruction::ClaimRewards");
                process_claim_rewards(accounts, program_id)
            }

            Instruction::ClosePool {} => {
                msg!("Instruction::ClosePool");
                process_close_pool(accounts, program_id)
            }

            Instruction::CloseUser {} => {
                msg!("Instruction::CloseUser");
                process_close_user(accounts, program_id)
            }
        }
    }
}
