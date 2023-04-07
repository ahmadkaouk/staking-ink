use openbrush::{
    contracts::traits::{psp22::PSP22Error, psp37::PSP37Error},
    traits::{AccountId, Balance, Timestamp},
};

#[cfg(feature = "std")]
#[openbrush::wrapper]
pub type StakingRef = dyn Staking;

#[openbrush::trait_definition]
pub trait Staking {
    /// Stakes the specified amount of tokens. The tokens are transferred from the caller's account.
    /// The caller must have approved the contract to transfer the specified amount of tokens.
    ///
    /// `amount` - The amount of tokens to stake.
    ///
    /// Returns `StakingError::InsufficientAllowance` if the caller has not approved the contract
    /// to transfer the specified amount of tokens.
    /// Returns `StakingError::InsufficientBalance` if the caller does not have enough tokens to
    /// stake.
    #[ink(message)]
    fn stake(&mut self, amount: Balance) -> Result<(), StakingError>;

    /// Unstakes the specified amount of tokens. The tokens are transferred to the caller's account.
    ///
    /// `amount` - The amount of tokens to unstake.
    ///
    /// Returns `StakingError::InsufficientBalance` if the caller does not have enough tokens to
    /// unstake.
    #[ink(message)]
    fn withdraw(&mut self, amount: Balance) -> Result<(), StakingError>;

    /// Claims the staking rewards for the caller. The rewards are transferred to the caller's
    /// account.
    ///
    /// Returns `StakingError::NoStakingRewards` if the caller has no staking rewards.
    #[ink(message)]
    fn get_reward(&mut self) -> Result<(), StakingError>;

    /// Returns the amount of tokens staked by the specified user. If the user has not staked any
    /// tokens, this method returns `0`.
    ///
    /// `staker` - The address of the user.
    #[ink(message)]
    fn balance_of(&self, staker: AccountId) -> Balance;

    /// Returns the total amount of tokens staked.
    #[ink(message)]
    fn total_supply(&self) -> Balance;
}

pub trait Internal {
    fn update_reward_rate(&mut self) -> Result<(), StakingError>;

    /// Returns the staking reward per token.
    fn reward_per_token(&self) -> Result<Balance, StakingError>;

    /// Updates the staking rewards for the specified user.
    fn update_reward(&mut self, account: AccountId) -> Result<(), StakingError>;

    fn earned(&self, account: AccountId) -> Result<Balance, StakingError>;

    fn last_time_reward_applicable(&self) -> Timestamp;

    fn update_reputation(&mut self, staker: AccountId) -> Result<(), StakingError>;
}

// Define an enum for the error codes that can be returned by the Staking trait.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum StakingError {
    /// The caller has not approved the contract to transfer the specified amount of tokens.
    InsufficientAllowance,
    /// The caller does not have enough tokens to stake.
    InsufficientBalance,
    /// The caller has no staking rewards.
    NoStakingRewards,
    /// The amount is zero.
    ZeroAmount,
    /// PSP22 error
    PSP22Error(PSP22Error),
    /// PSP37 error
    PSP37Error(PSP37Error),
    /// Overflow error
    OverflowError,
    /// Division by zero error
    DivideByZero,
}

impl From<PSP22Error> for StakingError {
    fn from(error: PSP22Error) -> Self {
        Self::PSP22Error(error)
    }
}

impl From<PSP37Error> for StakingError {
    fn from(error: PSP37Error) -> Self {
        Self::PSP37Error(error.into())
    }
}
