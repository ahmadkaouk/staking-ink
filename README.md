# Staking and Reputation Token System (Ink! and OpenBrush)
This project implements a staking and reputation token system using Ink! for Substrate-based blockchains. It includes three smart contracts, Staking, ReputationToken, and PSP22Token. Users can stake a PSP22 token to earn rewards and gain reputation points. The reputation points are represented by an NFT token following the ERC1155 standard (compatible with PSP37).

## Features
- Users can stake, unstake, and claim rewards in the Staking contract.
- The staking contract follows the Synthetix staking rewards model.
- The reputation token is an NFT token following the ERC1155 standard (PSP37 compatible).
- Reputation points are rewarded to stakers based on the staking amount and staking duration.
- Users can call the claim_reputation function to mint the reputation token.
- The reputation tokens are minted upon any reward distribution or when calling the claim reputation function.
- The ReputationToken contract handles infinite levels for reputation, each level requiring ten times more reputation points than the previous one.
- A PSP22Token contract representing the staking token.