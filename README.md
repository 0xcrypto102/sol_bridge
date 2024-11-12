# 🚀 Getting Started

### Prerequisites

- **Rust**: Make sure you have the latest version of Rust installed. You can install it using [rustup](https://rustup.rs/).
- **Anchor Framework**: This project uses [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html), a framework for Solana smart contracts.
- **Solana CLI**: Install the Solana CLI for interacting with the Solana network.
- **Node.js & Yarn**: For running deployment scripts and interacting with the smart contract.
  
# Solana-EVM Bridge Contract

This project is a Solana-EVM bridge contract designed to enable the transfer of assets and data between the Solana blockchain and any Ethereum-compatible EVM (Ethereum, Binance Smart Chain, Polygon, etc.). 
The bridge facilitates seamless interactions and cross-chain communication, allowing assets such as SPL tokens on Solana to be represented and utilized on EVM-compatible chains and vice versa.

## Overview

The **Solana-EVM Bridge Contract** enables interoperability between Solana and EVM-compatible blockchains by facilitating the transfer of tokens and data across these networks. This bridge uses smart contracts on both Solana and EVM chains to validate and verify transactions, ensuring secure cross-chain interactions.

## Architecture

The architecture of the Solana-EVM Bridge consists of:

1. **Solana Program (On-Chain)**:
   - A Rust-based Solana program responsible for handling asset transfers and storing metadata such as bridge events, token mappings, and user balances.

2. **EVM Smart Contract (On-Chain)**:
   - A Solidity smart contract on an EVM-compatible chain that interacts with Solana's bridge program. It facilitates token minting and burning, validates cross-chain transfers, and stores mapping data.

3. **Off-Chain Relayer**:
   - A Node.js backend service acting as a relayer that listens for bridge events on both Solana and the EVM chain, initiating the transfer process and validating proofs.

## Features

- **Asset Transfer**: Supports the transfer of SPL tokens from Solana to ERC-20 tokens on EVM chains and vice versa.
- **Cross-Chain Communication**: Verifies cross-chain transactions through a decentralized relayer network.
- **Security**: Ensures secure validation of transactions using cryptographic proofs.
- **Event Handling**: Handles events from both Solana and EVM chains, enabling real-time updates and notifications.

## Installation

### Prerequisites

- Node.js (v18 or higher)
- Rust and Solana CLI (for Solana program)
- Hardhat (for EVM smart contract)
- MongoDB (for storing bridge data, optional)

### Clone the Repository

```bash
git clone https://github.com/0xcrypto102/sol_bridge.git
cd sol_bridge

```

### Install Dependencies

```bash
npm install
```

## Deployment

### Deploy Solana Program

- Navigate to the Solana program directory and build the program
```bash
anchor build
```
- Deploy the program
```bash
solana program deploy ./target/deploy/sol_bridge.so
```

## Unit-test

### Solana Program Tests

Test File: tests/sol_bridge.ts

Commands:
```bash
anchor run test
```

### Test Cases

- Initialization:
  - Setting up the bridge with a protocol fee.
- Setting Protocol Fee:
  - Adjusting the fee that the bridge charges.
- Managing Tokens:
  - Adding bridgeable tokens with addToken.
  - Removing bridgeable tokens with removeToken.
- Liquidity Management:
  - Adding liquidity via addLiquidity.
  - Updating token balances using updateTokenBalance.
- Token Transfer:
  - Sending tokens with send.
- Handling Messages:
  - Processing incoming messages using messageReceive.
- Withdrawals:
  - Withdrawing tokens and protocol fees.


## 🤝 Contributing
Contributions are welcome! If you have suggestions for improving the project, please create an issue or submit a pull request.

Fork the project.
Create a new branch (git checkout -b feature/your-feature).
Commit your changes (git commit -m 'Add some feature').
Push to the branch (git push origin feature/your-feature).
Open a pull request.

## 📜 License
This project is licensed under the MIT License - see the LICENSE file for details.

## 🙋‍♂️ Support
If you encounter any issues or have questions about the project, feel free to open an issue or reach out to the maintainer.

## EVM Contract Source
```bash
https://github.com/0xGoldMaker/sol-bridge-evm-contract
```
