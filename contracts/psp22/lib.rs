#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// A PSP-22 compliant token.
#[openbrush::contract]
mod my_psp22 {

    use openbrush::contracts::psp22::*;
    use openbrush::traits::Storage;
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct StakingToken {
        #[storage_field]
        psp22: psp22::Data,
    }

    impl PSP22 for StakingToken {}

    impl StakingToken {
        /// Creates a new `StakingToken` instance.
        #[ink(constructor)]
        pub fn new(staking_contract: AccountId) -> Self {
            let mut instance = Self::default();

            // 1 billion with 18 decimals
            let initial_supply: Balance = 1_000_000_000 * 10u128.pow(18);
            // 70% of initial supply
            let staking_tokens: Balance = initial_supply * 70 / 100;

            instance
                ._mint_to(instance.env().caller(), initial_supply - staking_tokens)
                .expect("should mint");

            instance
                ._mint_to(staking_contract, staking_tokens)
                .expect("should mint staking tokens");

            instance
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {}

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {}
}
