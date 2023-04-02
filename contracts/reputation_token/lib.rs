#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod reputation_token {
    use openbrush::{
        contracts::psp37::*,
        traits::{Storage, String},
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct ReputationToken {
        #[storage_field]
        psp37: psp37::Data,
    }

    impl PSP37 for ReputationToken {}

    impl ReputationToken {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
