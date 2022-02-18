use crate::error::CustomError::InvalidInstruction;
use solana_program::program_error::ProgramError;
use std::convert::TryInto;
pub enum Instruction {
    InitializePool {
        reward_duration: u64,
        pool_nonce: u8,
        fund_amount: u64,
    },
    CreateUser {
        nonce: u8,
    },
    Stake {
        amount_to_deposit: u64,
    },
    Unstake {
        amount_to_withdraw: u64,
    },
    ClaimRewards {},
    ClosePool {},
    CloseUser {},
    FinalUnstake {},
    UpdateRates {
        rewards_per_slot: u64,
        max_reward_rate: u64,
        min_reward_rate: u64,
    },
}

impl Instruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        Ok(match input[0] {
            0 => Self::InitializePool {
                reward_duration: Self::unpack_to_u64(&input[1..9])?,
                pool_nonce: input[9],
                fund_amount: Self::unpack_to_u64(&input[10..18])?,
            },
            1 => Self::CreateUser { nonce: input[1] },
            2 => Self::Stake {
                amount_to_deposit: Self::unpack_to_u64(&input[1..9])?,
            },
            3 => Self::Unstake {
                amount_to_withdraw: Self::unpack_to_u64(&input[1..9])?,
            },

            4 => Self::ClaimRewards {},

            5 => Self::ClosePool {},

            6 => Self::CloseUser {},

            7 => Self::FinalUnstake {},

            8 => Self::UpdateRates {
                rewards_per_slot: Self::unpack_to_u64(&input[1..9])?,
                max_reward_rate: Self::unpack_to_u64(&input[9..17])?,
                min_reward_rate: Self::unpack_to_u64(&input[17..25])?,
            },

            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_to_u64(input: &[u8]) -> Result<u64, ProgramError> {
        let out_value = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(out_value)
    }
}