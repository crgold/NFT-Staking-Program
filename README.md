# Solana Program README

This README provides information about a Solana program for creating and managing NFTs, along with instructions for testing the program using a TypeScript test suite.

## Solana Program

The Solana program is designed to create, delegate, stake, and manage NFTs. It utilizes the Anchor framework and the Metaplex Token Metadata program for metadata management.

### Program Structure

The Solana program is structured as follows:

- The main program file is `create_nft.rs`, containing entry points for NFT creation, initialization, delegation, unstaking, rewards distribution, and record closure.
- The program is organized into distinct functions, each responsible for a specific operation.
- The program uses multiple accounts, associated tokens, and data structures for NFT and metadata management.

### Program Testing

The program includes a TypeScript test suite that validates various aspects of the program's functionality. The tests cover the following scenarios:

1. **Initialize Token Mint:** This test initializes a token mint and verifies its successful initialization.

2. **Create and Mint NFT:** This test creates and mints an NFT, associating it with metadata. It also checks the NFT's presence on the Solana blockchain.

3. **Delegate NFT:** The test delegates the NFT to a staking authority, preparing it for staking.

4. **Soft Stake NFT:** This test simulates staking the NFT, freezing it for staking purposes.

5. **Unstake NFT:** After staking, this test unfreezes the NFT, preparing it for unstaking.

6. **Undelegate NFT:** The test revokes the delegation of the NFT.

7. **Send Rewards:** This test calculates and sends rewards based on the NFT's staking duration.

8. **Close Record Account:** The test closes the NFT record account.

### Running the Tests

To run the tests:

1. Set up your Solana development environment.
2. Adjust the configuration variables in the TypeScript test file as needed.
3. Run the tests using a TypeScript test runner or `mocha`:

```shell
npx mocha path/to/test-file.ts
```

4. View test results and inspect transactions and NFTs on Solana Explorer using the provided URLs.

### Error Handling

The test suite includes assertions to check for specific conditions during testing. It helps validate the correctness of the program's operations.

### Contributions

Contributions to the test suite or the Solana program are welcome. If you encounter issues or want to enhance the program's test coverage, feel free to contribute to the codebase.

## About the TypeScript Test Suite

The TypeScript test suite is designed to test the Solana program's functionality, providing a way to verify its correctness and behavior.

### Running the Tests

To run the tests:

1. Set up the environment with Solana and Anchor.
2. Adjust the test configuration variables as needed.
3. Run the tests using a TypeScript test runner or `mocha`.

```shell
npx mocha path/to/test-file.ts
```

4. View test results, including transaction IDs and URLs for Solana Explorer.

### Viewing Transactions and NFTs

Throughout the test suite, transaction IDs (txid) and Solana Explorer URLs are provided to view transactions and NFTs. These links help inspect and verify the operations performed by the tests.

---

This README provides information on the Solana program and the TypeScript test suite designed to validate its functionality. The combined document offers insights into both the program and its testing procedures.
```

Please use this Markdown format for your README file, and adjust it as needed for your specific project.
