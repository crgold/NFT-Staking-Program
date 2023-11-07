import * as anchor from "@coral-xyz/anchor"
import 'dotenv/config'
import { Program } from "@coral-xyz/anchor"
import { CreateNft } from "../target/types/create_nft"
import { PublicKey, Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js'
// import { BN } from "bn.js"
import { MPL_TOKEN_METADATA_PROGRAM_ID as METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata'
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddress, ASSOCIATED_TOKEN_PROGRAM_ID, getAccount, getOrCreateAssociatedTokenAccount } from '@solana/spl-token'
import { assert } from "chai"
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes"

describe("create-nft-demo", async () => {
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.CreateNft as Program<CreateNft>
  const provider = anchor.AnchorProvider.env()

  const tokenMint = Keypair.generate()
  const nftMint = Keypair.generate()
  const user = Keypair.fromSecretKey(bs58.decode(process.env.PRIVATE_KEY))
  const userTokenAccount = await getAssociatedTokenAddress(nftMint.publicKey, user.publicKey)

  const [mintAuthority, mintAuthorityBump] = await PublicKey.findProgramAddressSync(
    [Buffer.from("mint-authority")],
    new PublicKey(program.programId)
  )
  
  const [metadata, metadataBump] = await PublicKey.findProgramAddressSync(
    [Buffer.from("metadata"), new PublicKey(METADATA_PROGRAM_ID).toBuffer(), nftMint.publicKey.toBuffer()],
    new PublicKey(METADATA_PROGRAM_ID)
  )

  const [tokenMetadata, tokenmMetadataBump] = await PublicKey.findProgramAddressSync(
    [Buffer.from("metadata"), new PublicKey(METADATA_PROGRAM_ID).toBuffer(), tokenMint.publicKey.toBuffer()],
    new PublicKey(METADATA_PROGRAM_ID)
  )

  const [masterEdition, masterBump] = await PublicKey.findProgramAddressSync(
    [Buffer.from("metadata"), new PublicKey(METADATA_PROGRAM_ID).toBuffer(), nftMint.publicKey.toBuffer(), Buffer.from("edition")],
    new PublicKey(METADATA_PROGRAM_ID)
  )

  const [stakingAuthority, stakingAuthorityBump] = await PublicKey.findProgramAddressSync(
    [Buffer.from("staking_Authority")],
    new PublicKey(program.programId)
  )

  const [nftRecord, nftRecordBump] = await PublicKey.findProgramAddressSync(
    [Buffer.from("nft_record"), user.publicKey.toBuffer(), nftMint.publicKey.toBuffer()],
    new PublicKey(program.programId)
  )

  it("Initialize Token Mint", async () => {

    const txid = await program.methods.initializeMint()
    .accounts({
      tokenMint: tokenMint.publicKey,
      mintAuthority: mintAuthority,
      metadataAccount: tokenMetadata,
      payer: user.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      metadataProgram: METADATA_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      rent: SYSVAR_RENT_PUBKEY
    })
    .signers([user, tokenMint])
    .rpc()
    console.log("Mint Initialized Successfully!");
    console.log("View transaction in explorer:");
    console.log(`https://solscan.io/tx/${txid}?cluster=devnet`);

    console.log("View Token in explorer:");
    console.log(`https://solscan.io/address/${tokenMint}?cluster=devnet`);

  })

  it("Create and mint NFT!", async () => {
    //await safeAirdrop(user.publicKey, provider.connection)
    const name = "my test NFT"
    const symbol = "DDR"
    const uri = "test-uri"

    const txid = await program.methods.createNft(name, symbol, uri)
    .accounts({
      user: user.publicKey,
      userTokenAccount: userTokenAccount,
      nftMint: nftMint.publicKey,
      metadataAccount: metadata,
      masterEdition: masterEdition,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      metadataProgram: METADATA_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      rent: SYSVAR_RENT_PUBKEY
    })
    .signers([user, nftMint])
    .rpc()
    console.log("View transaction in explorer:")
    console.log(`https://solscan.io/tx/${txid}?cluster=devnet`)

    console.log("View NFT in explorer:")
    console.log(`https://solscan.io/address/${nftMint.publicKey}?cluster=devnet`)

  })

  it("Delegate NFT", async () => {
    const txid = await program.methods.delegateNft()
    .accounts({
      user: user.publicKey,
      nftMint: nftMint.publicKey,
      userTokenAccount: userTokenAccount,
      tokenProgram: TOKEN_PROGRAM_ID,
      stakingAuthority: stakingAuthority
    })
    .signers([user])
    .rpc()

    console.log("NFT delegated and staked")
  })

  it("Soft stake NFT!", async () => {
    const txid = await program.methods.stakeNft()
    .accounts({
      user: user.publicKey,
      nftMint: nftMint.publicKey,
      userTokenAccount: userTokenAccount,
      masterEdition: masterEdition,
      tokenProgram: TOKEN_PROGRAM_ID,
      metadataProgram: METADATA_PROGRAM_ID,
      stakingAuthority: stakingAuthority,
      systemProgram: SystemProgram.programId,
      rent: SYSVAR_RENT_PUBKEY,
      nftRecord: nftRecord
    })
    .signers([])
    .rpc()
  })

  it("NFT should be frozen", async () => {
    const accountInfo = await getAccount(
      provider.connection,
      userTokenAccount
    )

    assert.isTrue(accountInfo.isFrozen);
  })

  it("Unstake NFT", async () => {
    const txid = await program.methods.unstakeNft()
    .accounts({
      user: user.publicKey,
      nftMint: nftMint.publicKey,
      userTokenAccount: userTokenAccount,
      nftRecord: nftRecord,
      stakingAuthority: stakingAuthority,
      masterEdition: masterEdition,
      metadataProgram: METADATA_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID
    })
    .signers([])
    .rpc()

    console.log("NFT Unstaked!");
  })

  it("Undelegate NFT", async () => {
    const txid = await program.methods.undelegateNft()
    .accounts({
      user: user.publicKey,
      nftMint: nftMint.publicKey,
      userTokenAccount: userTokenAccount,
      tokenProgram: TOKEN_PROGRAM_ID,
      stakingAuthority: stakingAuthority
    })
    .signers([user])
    .rpc()

    console.log("NFT revoked");
  })

  it("Send Rewards",async () => {
    const user_ata = await getOrCreateAssociatedTokenAccount(provider.connection, user, tokenMint.publicKey, user.publicKey);

    const txid = await program.methods.sendRewards()
    .accounts({
      tokenMint: tokenMint.publicKey,
      mintAuthority: mintAuthority,
      user: user.publicKey,
      userTokenAccount: user_ata.address,
      nftMint: nftMint.publicKey,
      nftRecord: nftRecord,
      rent: SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId    
    })
    .signers([user])
    .rpc()
    console.log("Rewards Sent!");
    console.log("View transaction in explorer:");
    console.log(`https://solscan.io/tx/${txid}?cluster=devnet`);
  })

  it("Close Record Account", async () => {
    const txid = await program.methods.closeRecord()
    .accounts({
      user: user.publicKey,
      nftRecord: nftRecord,
    })
    .signers([user])
    .rpc()

    console.log("NFT record closed!");
    console.log("View transaction in explorer:");
    console.log(`https://solscan.io/tx/${txid}?cluster=devnet`);
  })
})