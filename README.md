# Solana-EVM Bridge Contract

This project is a Solana-EVM bridge contract designed to enable the transfer of assets and data between the Solana blockchain and any Ethereum-compatible EVM (Ethereum, Binance Smart Chain, Polygon, etc.). 
The bridge facilitates seamless interactions and cross-chain communication, allowing assets such as SPL tokens on Solana to be represented and utilized on EVM-compatible chains and vice versa.

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Features](#features)
4. [Installation](#installation)
5. [Deployment](#deployment)
6. [Usage](#usage)
7. [Unit Tests](#unit-tests)
8. [Testing](#testing)
9. [Troubleshooting](#troubleshooting)
10. [Contributing](#contributing)
11. [License](#license)

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
cd solana-evm-bridge
