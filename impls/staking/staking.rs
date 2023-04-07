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

        self.update_reward(staker)?;
        PSP22Ref::transfer_from(&staking_token, staker, contract, amount, Vec::<u8>::new())?;

        let new_amount = self
            .data()
            .balances
            .get(&staker)
            .unwrap_or(0)
            .checked_add(amount)
            .ok_or(StakingError::OverflowError)?;

        self.data().balances.insert(&staker, &new_amount);
        self.data().total_supply = self
            .data()
            .total_supply
            .checked_add(amount)
            .ok_or(StakingError::OverflowError)?;

        Ok(())
    }

    default fn withdraw(&mut self, amount: Balance) -> Result<(), StakingError> {
        ensure!(amount > 0, StakingError::ZeroAmount);

        let staker = Self::env().caller();
        let staking_token = self.data().staking_token;
        let staked_amount = self.data().balances.get(&staker).unwrap_or(0);

        ensure!(staked_amount >= amount, StakingError::InsufficientBalance);

        // self.update_reward(staker)?;
        PSP22Ref::transfer(&staking_token, staker, amount, Vec::<u8>::new())?;

        self.data().balances.insert(
            &staker,
            &(staked_amount
                .checked_sub(amount)
                .ok_or(StakingError::OverflowError)?),
        );

        self.data().total_supply = self
            .data()
            .total_supply
            .checked_sub(amount)
            .ok_or(StakingError::OverflowError)?;

        Ok(())
    }

    default fn get_reward(&mut self) -> Result<(), StakingError> {
        let staker = Self::env().caller();

        self.update_reward(staker)?;

        let rewards = self.data().rewards.get(&staker).unwrap_or(0);
        if rewards > 0 {
            PSP22Ref::transfer(
                &self.data().staking_token,
                staker,
                rewards,
                Vec::<u8>::new(),
            )?;
            self.data().rewards.insert(&staker, &0);
        }
        Ok(())
    }

    default fn balance_of(&self, account: AccountId) -> Balance {
        self.data().balances.get(&account).unwrap_or(0)
    }

    default fn total_supply(&self) -> Balance {
        self.data().total_supply
    }
}
