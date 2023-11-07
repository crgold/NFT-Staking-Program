# Solana Program README

## Introduction

This is the README file for the Solana program `create_nft`, which is a program designed to create, manage, and stake NFTs (Non-Fungible Tokens) on the Solana blockchain. This program is built using the Anchor framework, and it offers a variety of functionalities, including NFT creation, initialization of token mints, NFT delegation, unstaking, sending rewards, and closing NFT records.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Program Structure](#program-structure)
- [Usage](#usage)
  - [Creating an NFT](#creating-an-nft)
  - [Initializing a Token Mint](#initializing-a-token-mint)
  - [Delegating an NFT](#delegating-an-nft)
  - [Unstaking an NFT](#unstaking-an-nft)
  - [Sending Rewards](#sending-rewards)
  - [Closing an NFT Record](#closing-an-nft-record)
- [Error Handling](#error-handling)
- [Contributing](#contributing)
- [License](#license)

## Prerequisites

Before using this Solana program, you should have the following prerequisites in place:

- A Solana development environment set up.
- The Anchor framework installed and configured.
- Knowledge of Solana program development and Rust programming.

## Program Structure

The Solana program is structured as follows:

- The program entry point is defined under the `create_nft` module.
- The program consists of several functions for different operations, including NFT creation, mint initialization, delegation, unstaking, reward distribution, and record closure.

## Usage

### Creating an NFT

To create an NFT, use the `create_nft` function. It takes parameters for the NFT's name, symbol, and URI. This function creates the NFT, initializes its metadata, and mints it to the user.

### Initializing a Token Mint

The `initialize_mint` function initializes a token mint. It sets the mint authority, metadata account, and other properties. This function is used for token mint setup.

### Delegating an NFT

The `delegate_nft` function allows users to delegate their NFTs to another party. It uses the `approve` function to grant delegation rights to the designated authority.

### Unstaking an NFT

The `unstake_nft` function is used to unstake a previously staked NFT. It revokes delegation using the `revoke` function.

### Sending Rewards

The `send_rewards` function calculates and sends rewards to the NFT holder based on the time they staked the NFT. It mints and transfers reward tokens to the user.

### Closing an NFT Record

The `close_record` function allows users to close their NFT records, indicating that they are no longer staking the NFT. This function doesn't perform any token transfers but can be used for record management.

## Error Handling

The program defines custom error codes, such as `TokenNotNFT` and `TokenAccountEmpty`, to handle specific error scenarios. Error messages are provided to help users understand the issues encountered during program execution.

## Contributing

Contributions to this Solana program are welcome. If you would like to contribute, please follow the standard open-source contribution guidelines. Fork the repository, make your changes, and submit a pull request for review.

## License

This Solana program is distributed under the open-source MIT License. You are free to use, modify, and distribute this program as per the terms of the license. For more details, please refer to the LICENSE file included in the repository.

Thank you for using the `create_nft` Solana program. If you have any questions or need further assistance, feel free to reach out to the program maintainers or the Solana community. Happy NFT management!
