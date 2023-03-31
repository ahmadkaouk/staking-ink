#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// A PSP-22 compliant token.
#[openbrush::contract]
mod my_psp22 {

    use openbrush::{
        contracts::psp22::extensions::metadata::*,
        traits::{Storage, String},
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct StakingToken {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP22 for StakingToken {}

    impl PSP22Metadata for StakingToken {}

    impl StakingToken {
        /// Creates a new `StakingToken` instance with the given `name` and `symbol`
        #[ink(constructor)]
        pub fn new(
            name: Option<String>,
            symbol: Option<String>,
            staking_contract: AccountId,
        ) -> Self {
            let mut instance = Self::default();

            instance.metadata.name = name;
            instance.metadata.symbol = symbol;
            instance.metadata.decimals = 18;

            // 1 billion with 18 decimals
            let initial_supply = 1_000_000_000 * 10u128.pow(18);
            // 70% of initial supply
            let staking_tokens = initial_supply * 70 / 100;

            assert!(instance
                ._mint_to(instance.env().caller(), initial_supply - staking_tokens)
                .is_ok());

            assert!(instance._mint_to(staking_contract, staking_tokens).is_ok());

            instance
        }
    }
}
