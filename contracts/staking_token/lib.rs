#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// A PSP-22 compliant staking token with metadata.
#[openbrush::contract]
pub mod token {
    use openbrush::{
        contracts::psp22::extensions::metadata::*,
        traits::{self, Storage},
    };

    /// The main storage structure of the `StakingTokenContract` contract.
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct StakingTokenContract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    /// Implementation of the PSP22 standard for this contract.
    impl PSP22 for StakingTokenContract {}

    /// Implementation of the PSP22Metadata extension for this contract.
    impl PSP22Metadata for StakingTokenContract {}

    /// Implementation of the `StakingTokenContract` contract.
    impl StakingTokenContract {
        /// Creates a new `StakingTokenContract` instance with the given `name`,`symbol`,
        /// `decimals` and `initial_supply`.
        #[ink(constructor)]
        pub fn new(
            name: Option<traits::String>,
            symbol: Option<traits::String>,
            decimals: u8,
            initial_supply: Balance,
        ) -> Self {
            let mut instance = Self::default();

            instance.metadata.name = name;
            instance.metadata.symbol = symbol;
            instance.metadata.decimals = decimals;

            assert!(
                instance
                    ._mint_to(instance.env().caller(), initial_supply)
                    .is_ok(),
                "Failed to mint tokens to the contract creator"
            );

            instance
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use openbrush::test_utils::*;

        const INITIAL_SUPPLY: u128 = 1_000_000_000 * 10u128.pow(18);

        #[ink::test]
        fn constructor_sets_name_symbol_and_decimals() {
            let name = Some(traits::String::from("My Staking Token"));
            let symbol = Some(traits::String::from("MST"));
            let instance =
                StakingTokenContract::new(name.clone(), symbol.clone(), 18, INITIAL_SUPPLY);

            assert_eq!(instance.token_name(), name);
            assert_eq!(instance.token_symbol(), symbol);
            assert_eq!(instance.token_decimals(), 18);
        }

        #[ink::test]
        fn constructor_distributes_tokens_correctly() {
            let name = Some(traits::String::from("My Staking Token"));
            let symbol = Some(traits::String::from("MST"));
            let instance =
                StakingTokenContract::new(name.clone(), symbol.clone(), 18, INITIAL_SUPPLY);
            let owner = accounts().alice;

            assert_eq!(instance.total_supply(), INITIAL_SUPPLY);
            assert_eq!(instance.balance_of(owner), INITIAL_SUPPLY);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {

        use super::*;
        use ink_e2e::build_message;
        use openbrush::contracts::psp22::{
            extensions::metadata::psp22metadata_external::PSP22Metadata, psp22_external::PSP22,
        };

        const INITIAL_SUPPLY: u128 = 1_000_000_000 * 10u128.pow(18);

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its constructor.
        #[ink_e2e::test]
        async fn instantiation_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = StakingTokenContractRef::new(
                Some(traits::String::from("My Staking Token")),
                Some(traits::String::from("MST")),
                18,
                INITIAL_SUPPLY,
            );

            // Instantiate the contract
            let contract_account_id = client
                .instantiate("staking_token", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Check Token Name
            let token_name = build_message::<StakingTokenContractRef>(contract_account_id.clone())
                .call(|token| token.token_name());
            assert_eq!(
                client
                    .call_dry_run(&ink_e2e::alice(), &token_name, 0, None)
                    .await
                    .return_value(),
                Some(traits::String::from("My Staking Token"))
            );

            // Check Token Symbol
            let token_symbol =
                build_message::<StakingTokenContractRef>(contract_account_id.clone())
                    .call(|token| token.token_symbol());
            assert_eq!(
                client
                    .call_dry_run(&ink_e2e::alice(), &token_symbol, 0, None)
                    .await
                    .return_value(),
                Some(traits::String::from("MST"))
            );

            // Check Token Decimals
            let token_decimals =
                build_message::<StakingTokenContractRef>(contract_account_id.clone())
                    .call(|token| token.token_decimals());
            assert_eq!(
                client
                    .call_dry_run(&ink_e2e::alice(), &token_decimals, 0, None)
                    .await
                    .return_value(),
                18
            );

            // Check Total Supply
            let total_supply =
                build_message::<StakingTokenContractRef>(contract_account_id.clone())
                    .call(|token| token.total_supply());
            assert_eq!(
                client
                    .call_dry_run(&ink_e2e::alice(), &total_supply, 0, None)
                    .await
                    .return_value(),
                INITIAL_SUPPLY
            );

            // Check Balance of Contract Owner (Alice)
            let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let alice_balance =
                build_message::<StakingTokenContractRef>(contract_account_id.clone())
                    .call(|token| token.balance_of(alice_account));
            assert_eq!(
                client
                    .call_dry_run(&ink_e2e::bob(), &alice_balance, 0, None)
                    .await
                    .return_value(),
                INITIAL_SUPPLY
            );

            Ok(())
        }
    }
}
