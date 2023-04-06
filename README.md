# Staking and Reputation Token System (Ink! and OpenBrush)
This project implements a staking and reputation token system using Ink! for Substrate-based blockchains. It includes three smart contracts, Staking, ReputationToken, and PSP22Token. Users can stake a PSP22 token to earn rewards and gain reputation points. The reputation points are represented by an NFT token following the ERC1155 standard (compatible with PSP37).

## Features
- Users can stake PSP22 tokens to participate in the staking process.
- Stakers can earn rewards based on the amount and duration of staking.
- The staking contract follows the Synthetix staking rewards model.
- Users can unstake their tokens and claim rewards.
- Reputation tokens are rewarded to users based on certain milestones.
- The contract is built using Ink! and OpenBrush.

## Overview
The application consists of several components:

### PSP22 (ERC20) Token
- Initial supply of 1 billion tokens
- 70% of the tokens will be sent to the staking contract
- 18 decimals

## Staking Contract
- Allows users to stake the created PSP22 token
- Stakers will get their share of the tokens inside the staking contract, with:
    - 50% of the tokens being released during the first 365 days (35% of the initial supply)
    - A halving occurring each 365 days
    - 25% of the tokens in the staking contract being distributed during the next 365 days (17.5% of the initial supply)
    - 12.5% of the tokens in the staking contract being distributed in the next 365 days, and so on
- Users can stake, unstake, and claim rewards
- Reward distribution happens on any of these actions

## Reputation Token
- NFT token - Multi-token (PSP37/ERC1155)
- Rewarded to stakers on certain milestones
- Staking 1 token (10**18 tokens) for 1 day increases reputation by 1
- Reputation levels:
    - Level 1: 1 billion reputation
    - Level 2: 10 billion reputation
    - Level 3: 100 billion reputation, and so on
    - An infinite amount of levels can be created
- Users can call the claim reputation function, which mints the reputation token to them
- Tokens will be minted on any of the reward distributions or upon calling the claim reputation function
## TODO
> WARNING: The rewards distribution functionality has not been thoroughly tested yet.

- [ ] Add tests for rewards distribution and reputation computation in the staking contract
- [ ] Add e2e tests for the reputation token contract