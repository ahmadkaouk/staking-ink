#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod staking {
    use ink::ToAccountId;
    use openbrush::traits::{Storage, String};
    use staking_dapp::impls::*;
    use staking_token::token::StakingTokenContractRef;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct StakingContract {
        #[storage_field]
        staking: staking::Data,
    }

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
            instance.staking.start_time = instance.env().block_timestamp();

            instance
        }

        /// Stakes the specified amount of tokens.
        #[ink(message)]
        pub fn stake(&mut self, amount: Balance) {}
    }
}
