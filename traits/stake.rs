use openbrush::traits::{AccountId, Balance, Timestamp};

#[cfg(feature = "std")]
use ink::storage::traits::StorageLayout;

#[derive(Debug, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub struct StakeInfo {
    pub staker: AccountId,
    pub amount: Balance,
    pub start_time: Timestamp,
}

impl Default for StakeInfo {
    fn default() -> Self {
        Self {
            staker: [0u8; 32].into(),
            amount: Default::default(),
            start_time: Default::default(),
        }
    }
}

#[openbrush::wrapper]
pub type StakingRef = dyn Staking;

#[openbrush::trait_definition]
pub trait Staking {
    /// Stakes the specified amount of tokens.
    #[ink(message)]
    fn stake(&mut self, amount: Balance);

    #[ink(message)]
    fn unstake(&mut self, amount: Balance);

    #[ink(message)]
    fn withdraw(&mut self);

    #[ink(message)]
    fn get_stake_info(&self, staker: AccountId) -> StakeInfo;
}

/// Enum for the error codes that can be returned by the `Stake` trait.
pub enum StakingError {
}
