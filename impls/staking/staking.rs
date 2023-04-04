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

impl<T: Storage<data::Data>> Staking for T {
    default fn stake(&mut self, amount: Balance) -> Result<(), StakingError> {
        let staker = Self::env().caller();
        let contract = Self::env().account_id();
        let staking_token = self.data().staking_token;

        // ensure the staker gave allowance to the contract
        ensure!(
            PSP22Ref::allowance(&staking_token, staker, contract) >= amount,
            StakingError::InsufficientAllowance
        );

        // ensure the user has enough tokens
        ensure!(
            PSP22Ref::balance_of(&staking_token, staker) >= amount,
            StakingError::InsufficientBalance
        );

        // Transfer tokens from the caller to the contract
        PSP22Ref::transfer_from(&staking_token, staker, contract, amount, Vec::<u8>::new())?;

        // Update the total amount of tokens staked
        self.data().total_staked += amount;
        // Update the amount of tokens staked by the staker
        let new_amount = self.data().stakers.get(&staker).unwrap_or(0) + amount;
        self.data().stakers.insert(&staker, &new_amount);
        Ok(())
    }

    default fn unstake(&mut self, amount: Balance) -> Result<(), StakingError> {
        let staker = Self::env().caller();
        let contract = Self::env().account_id();
        let staking_token = self.data().staking_token;

        // ensure the user has enough tokens
        let staked_amount = self.data().stakers.get(&staker).unwrap_or(0);
        ensure!(staked_amount >= amount, StakingError::InsufficientBalance);

        // Transfer tokens from the contract to the caller
        PSP22Ref::transfer_from(&staking_token, contract, staker, amount, Vec::<u8>::new())?;

        // Update the amount of tokens staked by the caller
        self.data()
            .stakers
            .insert(&staker, &(staked_amount - amount));

        Ok(())
    }

    default fn claim_rewards(&mut self) -> Result<(), StakingError> {
        Ok(())
    }

    default fn staked_amount(&self, account: AccountId) -> Balance {
        self.data().stakers.get(&account).unwrap_or(0)
    }

    default fn total_staked(&self) -> Balance {
        self.data().total_staked
    }
}
