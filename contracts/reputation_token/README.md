# Reputation Token

The Reputation Token contract is an NFT token (PSP37) that rewards stakers based on the amount of tokens staked and the duration of staking. Users earn reputation points and can claim reputation tokens, which represent their reputation level.

## Features

- Multitoken (PSP37) support
- Rewarding reputation tokens to stakers based on certain milestones
- Incrementing reputation levels with an infinite number of levels
- Staking 1 token (10^18 tokens) for 1 day increases reputation by 1 point
- Users can call the claim_reputation function to mint reputation tokens

## Reputation Levels

- Level 1: 1 billion reputation points
- Level 2: 10 billion reputation points
- Level 3: 100 billion reputation points
...

## Usage

### Building

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
