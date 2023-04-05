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
    /// Period Start timestamp
    pub period_start: Timestamp,
    /// Period Finish timestamp
    pub period_finish: Timestamp,
    /// Reward Rate how much reward token will be distributed per second
    pub reward_rate: Balance,
    /// Rewards duration in seconds
    pub rewards_duration: Timestamp,
    /// Last Update timestamp
    pub last_update_time: Timestamp,
    /// Reward Per Token Stored Accumulated reward per token, times 1e18.
    pub reward_per_token_stored: Balance,
    /// User Reward Per Token Paid
    pub user_reward_per_token_paid: Mapping<AccountId, Balance>,
    /// User Reward
    pub rewards: Mapping<AccountId, Balance>,
    /// The sum of all staked amounts of all users.
    pub total_supply: Balance,
    /// The mapping from user addresses to their staked amounts.
    pub balances: Mapping<AccountId, Balance>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            staking_token: [0u8; 32].into(),
            period_start: Timestamp::default(),
            period_finish: Timestamp::default(),
            reward_rate: Balance::default(),
            rewards_duration: Timestamp::default(),
            last_update_time: Timestamp::default(),
            reward_per_token_stored: Balance::default(),
            user_reward_per_token_paid: Default::default(),
            rewards: Default::default(),
            total_supply: Balance::default(),
            balances: Default::default(),
        }
    }
}
