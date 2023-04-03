# My Staking Token
This repository contains a PSP-22 compliant staking token with metadata implemented using the OpenBrush library.

## Overview
My Staking Token is a Substrate-based smart contract that follows the PSP-22 (similar to ERC20) token standard with additional metadata, built with openbrush. The initial supply of the token is 1 billion with 18 decimal places. During the token creation, 70% of the initial supply is sent to the specified staking contract address, while the remaining 30% is assigned to the contract creator.

## Features
- PSP-22 compliant
- Metadata (token name, symbol, and decimals)
- Initial supply of 1 billion
- 18 decimal places
- Staking contract allocation (70% of the initial supply)

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
CONTRACTS_NODE=<path_to_binary> cargo +nightly test --features e2e-tests
```
