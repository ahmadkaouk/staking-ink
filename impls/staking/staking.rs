use crate::{impls::staking::data, traits::staking::*};
use ink::prelude::vec::Vec;
use openbrush::{
    contracts::traits::psp22::PSP22Ref,
    traits::{AccountId, Balance, Storage},
};

/// Macro ensures a condition is met, otherwise it returns an error.
macro_rules! ensure {
    ($condition:expr, $error:expr) => {
        if !$condition {
            return Err($error);
        }
    };
}

impl<T> Staking for T
where
    T: Storage<data::Data> + Internal,
{
    default fn stake(&mut self, amount: Balance) -> Result<(), StakingError> {
        ensure!(amount > 0, StakingError::ZeroAmount);

        let staker = Self::env().caller();
        let contract = Self::env().account_id();
        let staking_token = self.data().staking_token;

        ensure!(
            PSP22Ref::allowance(&staking_token, staker, contract) >= amount,
            StakingError::InsufficientAllowance
        );

        ensure!(
            PSP22Ref::balance_of(&staking_token, staker) >= amount,
            StakingError::InsufficientBalance
        );

        self.update_reward(staker);
        PSP22Ref::transfer_from(&staking_token, staker, contract, amount, Vec::<u8>::new())?;

        let new_amount = self.data().balances.get(&staker).unwrap_or(0) + amount;
        self.data().balances.insert(&staker, &new_amount);
        self.data().total_supply += amount;

        Ok(())
    }

    default fn withdraw(&mut self, amount: Balance) -> Result<(), StakingError> {
        ensure!(amount > 0, StakingError::ZeroAmount);

        let staker = Self::env().caller();
        let contract = Self::env().account_id();
        let staking_token = self.data().staking_token;
        let staked_amount = self.data().balances.get(&staker).unwrap_or(0);

        ensure!(staked_amount >= amount, StakingError::InsufficientBalance);

        self.update_reward(staker);
        PSP22Ref::transfer_from(&staking_token, contract, staker, amount, Vec::<u8>::new())?;

        self.data()
            .balances
            .insert(&staker, &(staked_amount - amount));

        Ok(())
    }

    default fn get_reward(&mut self) -> Result<(), StakingError> {
        let staker = Self::env().caller();
        let rewards = self.data().rewards.get(&staker).unwrap_or(0);
        ensure!(rewards > 0, StakingError::NoStakingRewards);
        self.update_reward(staker);
        self.data().rewards.insert(&staker, &0);
        Ok(())
    }

    default fn exit(&mut self) -> Result<(), StakingError> {
        let staker = Self::env().caller();
        let staked_amount = self.data().balances.get(&staker).unwrap_or(0);
        self.withdraw(staked_amount)?;
        self.get_reward()?;
        self.data().rewards.insert(&staker, &0);
        self.data().balances.insert(&staker, &0);
        Ok(())
    }

    default fn staked_amount(&self, account: AccountId) -> Balance {
        self.data().balances.get(&account).unwrap_or(0)
    }

    default fn total_supply(&self) -> Balance {
        self.data().total_supply
    }
}