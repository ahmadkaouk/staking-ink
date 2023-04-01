#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// A PSP-22 compliant staking token with metadata.
///
/// This token has an initial supply of 1 billion tokens with 18 decimal places.
/// During the token creation, 70% of the initial supply is sent to the specified
/// staking contract address, while the remaining 30% is assigned to the contract
/// creator.
#[openbrush::contract]
mod my_staking_token {
    use openbrush::{
        contracts::psp22::extensions::metadata::*,
        traits::{Storage, String},
    };

    /// The initial supply of the token: 1 billion (with 18 decimal places).
    const INITIAL_SUPPLY: u128 = 1_000_000_000 * 10u128.pow(18);
    /// The percentage of tokens sent to the staking contract: 70%.
    const STAKING_ALLOCATION: u128 = 70;

    /// The main storage structure of the `MyStakingToken` contract.
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct MyStakingToken {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    /// Implementation of the PSP22 standard for this contract.
    impl PSP22 for MyStakingToken {}

    /// Implementation of the PSP22Metadata extension for this contract.
    impl PSP22Metadata for MyStakingToken {}

    /// Implementation of the `MyStakingToken` contract.
    impl MyStakingToken {
        /// Creates a new `MyStakingToken` instance with the given `name` and `symbol`.
        ///
        /// The `staking_contract_address` parameter is the address where 70% of the
        /// initial token supply will be sent.
        #[ink(constructor)]
        pub fn new(
            name: Option<String>,
            symbol: Option<String>,
            staking_contract_address: AccountId,
        ) -> Self {
            let mut instance = Self::default();

            instance.metadata.name = name;
            instance.metadata.symbol = symbol;
            instance.metadata.decimals = 18;

            let staking_tokens = INITIAL_SUPPLY * STAKING_ALLOCATION / 100;

            assert!(
                instance
                    ._mint_to(instance.env().caller(), INITIAL_SUPPLY - staking_tokens)
                    .is_ok(),
                "Failed to mint tokens to the contract creator"
            );

            assert!(
                instance
                    ._mint_to(staking_contract_address, staking_tokens)
                    .is_ok(),
                "Failed to mint tokens to the staking contract"
            );

            instance
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_sets_name_symbol_and_decimals() {
            let name = Some(String::from("My Staking Token"));
            let symbol = Some(String::from("MST"));
            let staking_contract_address = AccountId::from([0x2; 32]);
            let instance =
                MyStakingToken::new(name.clone(), symbol.clone(), staking_contract_address);

            assert_eq!(instance.token_name(), name);
            assert_eq!(instance.token_symbol(), symbol);
            assert_eq!(instance.token_decimals(), 18);
        }

        /// Test that the `MyStakingToken` constructor distributes tokens correctly,
        /// assigning 70% to the staking contract and 30% to the contract creator.
        #[ink::test]
        fn constructor_distributes_tokens_correctly() {
            let name = Some(String::from("My Staking Token"));
            let symbol = Some(String::from("MST"));
            let staking_contract_address = AccountId::from([0x2; 32]);
            let instance =
                MyStakingToken::new(name.clone(), symbol.clone(), staking_contract_address);
            let account = AccountId::from([0x1; 32]);
            let staking_tokens = INITIAL_SUPPLY * STAKING_ALLOCATION / 100;
            let creator_tokens = INITIAL_SUPPLY - staking_tokens;

            assert_eq!(instance.total_supply(), INITIAL_SUPPLY);
            assert_eq!(instance.balance_of(account), creator_tokens);
            assert_eq!(
                instance.balance_of(staking_contract_address),
                staking_tokens
            );
        }
    }
}
