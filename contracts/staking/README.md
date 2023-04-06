# Staking Contract (Ink! and OpenBrush)
This repository contains a staking contract for Substrate-based blockchains using Ink! and OpenBrush. The contract allows users to stake and unstake a PSP22 token and earn rewards for staking.

## Features
- Users can stake PSP22 tokens to participate in the staking process.
- Stakers can earn rewards based on the amount and duration of staking.
- The staking contract follows the Synthetix staking rewards model.
- Users can unstake their tokens and claim rewards.
- The contract is built using Ink! and OpenBrush

## Usage
After deploying the staking contract, users can interact with it using Polkadot JS API. Users can stake tokens, unstake tokens, and claim rewards.

1. To stake tokens, call the `stake` function with the amount of tokens to stake.
2. To unstake tokens, call the `withdraw` function with the amount of tokens to unstake.
3. To claim rewards, call the `get_reward` function.
4. To claim reputation tokens, call the `claim_reputation` function.


## Building
To build the contract, run:

```bash
cargo +nightly contract build --release
```
This will generate a .contract file in the target directory.

### Testing
To run the unit tests, execute:

```bash
cargo +nightly test
```
To run integration test you need to start the node with contract-pallet. check [here](https://github.com/paritytech/substrate-contracts-node) for more details.

For example, to run the integration tests with a local node binary, run:

```bash
CONTRACTS_NODE=<path_to_contracts_node_binary> cargo +nightly test --features e2e-tests
```