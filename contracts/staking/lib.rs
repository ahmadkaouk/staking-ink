#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod staking {
    use core::ops::Mul;
    use ink::ToAccountId;
    use openbrush::traits::{DefaultEnv, Storage, String};
    use staking_dapp::{
        impls::staking::{data, staking::*},
        traits::staking::*,
    };
    use staking_token::token::StakingTokenContractRef;

    const SECONDS_PER_YEAR: u64 = 31_536_000;
    const INITIAL_REWARD_RATE: u128 = 50;
    const STAKING_ALLOCATION: u128 = 70;
    const INITIAL_SUPPLY: u128 =
        100_000_000_000_000_000_000_000_000_000u128 * STAKING_ALLOCATION / 100;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct StakingContract {
        #[storage_field]
        staking: data::Data,
    }

    impl Internal for StakingContract {
        fn update_reward_rate(&mut self) {
            let now = Self::env().block_timestamp();
            let years_elapsed = (now - self.staking.period_start) / SECONDS_PER_YEAR;
            self.staking.period_finish =
                self.staking.period_start + years_elapsed * SECONDS_PER_YEAR;
            self.staking.reward_rate = (INITIAL_REWARD_RATE >> years_elapsed) as u128
                * INITIAL_SUPPLY
                / SECONDS_PER_YEAR as u128;
        }

        fn reward_per_token(&self) -> u128 {
            if self.staking.total_supply == 0 {
                self.staking.reward_per_token_stored
            } else {
                let delta_time = self.last_time_reward_applicable() - self.staking.last_update_time;
                let reward =
                    delta_time as u128 * self.staking.reward_rate * 1_000_000_000_000_000_000u128;
                self.staking.reward_per_token_stored + (reward / self.staking.total_supply)
            }
        }

        fn update_reward(&mut self, staker: AccountId) {
            self.update_reward_rate();
            self.staking.reward_per_token_stored = self.reward_per_token();
            self.staking.last_update_time = self.last_time_reward_applicable();
            self.staking.rewards.insert(&staker, &self.earned(staker));
            self.staking
                .user_reward_per_token_paid
                .insert(&staker, &self.staking.reward_per_token_stored);
        }

        fn earned(&self, staker: AccountId) -> Balance {
            self.staking.balances.get(&staker).unwrap_or(0).mul(
                self.reward_per_token()
                    - self
                        .staking
                        .user_reward_per_token_paid
                        .get(&staker)
                        .unwrap_or(0),
            ) / 1_000_000_000_000_000_000u128
                + self.staking.rewards.get(&staker).unwrap_or(0)
        }

        fn last_time_reward_applicable(&self) -> Timestamp {
            let now = Self::env().block_timestamp();
            if now < self.staking.period_finish {
                now
            } else {
                self.staking.period_finish
            }
        }
    }

    impl Staking for StakingContract {}

    impl StakingContract {
        #[ink(constructor)]
        pub fn new(staking_token_hash: Hash) -> Self {
            let mut instance = Self::default();

            // Get current contract address
            let contract_address = instance.env().account_id();

            // instantiate the staking token contract
            let staking_token = StakingTokenContractRef::new(
                Some(String::from("Staking Token")),
                Some(String::from("STK")),
                contract_address,
            )
            .endowment(0)
            .code_hash(staking_token_hash)
            .salt_bytes(&[0xDE, 0xAD, 0xBE, 0xEF])
            .instantiate();

            instance.staking.staking_token = staking_token.to_account_id();
            instance.staking.period_start = instance.env().block_timestamp();
            instance
        }
    }
}
