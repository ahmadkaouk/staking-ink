use crate::traits::staking::*;
use openbrush::{
    storage::Mapping,
    traits::{AccountId, Balance, Timestamp},
};

const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    /// The address of the staking token contract.
    pub staking_token: AccountId,
    /// The total amount of tokens staked.
    pub total_staked: Balance,
    /// The total amount of tokens staked by each user.
    pub stakers: Mapping<AccountId, Balance>,
    /// The total amount of tokens staked by each user.
    pub staking_rewards: Mapping<AccountId, Balance>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            staking_token: [0u8; 32].into(),
            total_staked: Default::default(),
            stakers: Default::default(),
            staking_rewards: Default::default(),
            // start_time: Default::default(),
            // epoch_duration: Default::default(),
        }
    }
}
