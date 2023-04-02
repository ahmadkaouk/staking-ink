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
        traits::{Storage, String as OBString},
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
            name: Option<OBString>,
            symbol: Option<OBString>,
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
        use openbrush::test_utils::*;

        #[ink::test]
        fn constructor_sets_name_symbol_and_decimals() {
            let name = Some(OBString::from("My Staking Token"));
            let symbol = Some(OBString::from("MST"));
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
            let name = Some(OBString::from("My Staking Token"));
            let symbol = Some(OBString::from("MST"));
            let staking_contract_addr = accounts().bob;
            let instance = MyStakingToken::new(name.clone(), symbol.clone(), staking_contract_addr);
            let owner = accounts().alice;
            let staking_tokens = INITIAL_SUPPLY * STAKING_ALLOCATION / 100;
            let creator_tokens = INITIAL_SUPPLY - staking_tokens;

            assert_eq!(instance.total_supply(), INITIAL_SUPPLY);
            assert_eq!(instance.balance_of(owner), creator_tokens);
            assert_eq!(instance.balance_of(staking_contract_addr), staking_tokens);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {

        use super::*;
        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;
        use openbrush::contracts::psp22::{
            extensions::metadata::psp22metadata_external::PSP22Metadata, psp22_external::PSP22,
        };

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its constructor.
        #[ink_e2e::test]
        async fn instantiation_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let staking_contract_addr = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
            let constructor = MyStakingTokenRef::new(
                Some(OBString::from("My Staking Token")),
                Some(OBString::from("MST")),
                staking_contract_addr,
            );

            // When
            let contract_account_id = client
                .instantiate("staking_token", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Check Token Name
            let token_name = build_message::<MyStakingTokenRef>(contract_account_id.clone())
                .call(|token| token.token_name());
            assert_eq!(
                client
                    .call_dry_run(&ink_e2e::alice(), &token_name, 0, None)
                    .await
                    .return_value(),
                Some(OBString::from("My Staking Token"))
            );

            // Check Token Symbol
            let token_symbol = build_message::<MyStakingTokenRef>(contract_account_id.clone())
                .call(|token| token.token_symbol());
            assert_eq!(
                client
                    .call_dry_run(&ink_e2e::alice(), &token_symbol, 0, None)
                    .await
                    .return_value(),
                Some(OBString::from("MST"))
            );

            // Check Token Decimals
            let token_decimals = build_message::<MyStakingTokenRef>(contract_account_id.clone())
                .call(|token| token.token_decimals());
            assert_eq!(
                client
                    .call_dry_run(&ink_e2e::alice(), &token_decimals, 0, None)
                    .await
                    .return_value(),
                18
            );

            // Check Total Supply
            let total_supply = build_message::<MyStakingTokenRef>(contract_account_id.clone())
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
            let alice_balance = build_message::<MyStakingTokenRef>(contract_account_id.clone())
                .call(|token| token.balance_of(alice_account));
            assert_eq!(
                client
                    .call_dry_run(&ink_e2e::bob(), &alice_balance, 0, None)
                    .await
                    .return_value(),
                INITIAL_SUPPLY - (INITIAL_SUPPLY * STAKING_ALLOCATION / 100)
            );

            // Check Balance of Staking Contract (Bob)
            let bob_balance = build_message::<MyStakingTokenRef>(contract_account_id.clone())
                .call(|token| token.balance_of(staking_contract_addr));
            assert_eq!(
                client
                    .call_dry_run(&ink_e2e::alice(), &bob_balance, 0, None)
                    .await
                    .return_value(),
                INITIAL_SUPPLY * STAKING_ALLOCATION / 100
            );

            Ok(())
        }
    }
}
