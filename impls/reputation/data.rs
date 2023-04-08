use openbrush::{
    storage::Mapping,
    traits::{AccountId, Timestamp},
};

const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    /// The address of the staking token contract.
    pub reputation_token: AccountId,
    /// Mapping from account to last update time of reputation
    pub reputation_last_update: Mapping<AccountId, Timestamp>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            reputation_token: [0u8; 32].into(),
            reputation_last_update: Default::default(),
        }
    }
}
