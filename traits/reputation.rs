use openbrush::contracts::traits::{access_control::*, psp37::*};
use openbrush::{contracts::traits::psp37::PSP37Error, traits::AccountId};

#[openbrush::wrapper]
pub type ReputationRef = dyn Reputation + PSP37 + AccessControl;

#[openbrush::trait_definition]
pub trait Reputation: PSP37 + AccessControl {
    /// Update reputation of the account and mint tokens
    #[ink(message)]
    fn update_reputation(
        &mut self,
        staker: AccountId,
        new_reputation: u128,
    ) -> Result<(), PSP37Error>;
}

pub trait Internal {
    /// Returns the level of the reputation
    fn get_level(reputation: u128) -> u32;
}
