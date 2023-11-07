use anchor_lang::{
    prelude::*,
    solana_program::clock::Clock
};
use anchor_spl::{token::{Approve, approve, Revoke, revoke, TokenAccount, Token, Mint, MintTo, mint_to},
                associated_token::AssociatedToken, 
                metadata::MasterEditionAccount
};
use mpl_token_metadata::{
    ID as METADATA_PROGRAM_ID,
    instructions::{FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts, ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts, CreateMasterEditionV3Cpi, CreateMasterEditionV3CpiAccounts, CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3Cpi, CreateMetadataAccountV3InstructionArgs, CreateMetadataAccountV3CpiAccounts},
    types::{Creator, DataV2}
};

declare_id!("DtwR5igWmGM14GSgS844Szeh9qvSZt4HPie96bw7Dbqv");

#[program]
pub mod create_nft {
    use super::*;

    pub fn create_nft(ctx: Context<CreateNFT>, name: String, symbol: String, uri: String) -> Result<()> {

        // build our instruction using the accounts and data structs above and invoke it
        CreateMetadataAccountV3Cpi::new(
            &ctx.accounts.metadata_program.to_account_info(),
            CreateMetadataAccountV3CpiAccounts {
                metadata: &ctx.accounts.metadata_account.to_account_info(),
                mint: &ctx.accounts.nft_mint.to_account_info(),
                mint_authority: &ctx.accounts.user.to_account_info(),
                payer: &ctx.accounts.user.to_account_info(),
                update_authority: (&ctx.accounts.user.to_account_info(), false),
                system_program: &ctx.accounts.system_program.to_account_info(),
                rent: Some(&ctx.accounts.rent.to_account_info())
            },
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name: name,
                    symbol: symbol,
                    uri: uri,
                    seller_fee_basis_points: 0,
                    creators: Some(vec![
                        Creator {
                            address: ctx.accounts.user.key(),
                            verified: true,
                            share: 100,
                        }]),
                    collection: None,
                    uses: None
                },
                is_mutable: false,
                collection_details: None
            }
        ).invoke()?;

        // mint nft to user
        mint_to(
            ctx.accounts.mint_ctx(),
            1
        )?;

        // create master account
        CreateMasterEditionV3Cpi::new(
            &ctx.accounts.metadata_program.to_account_info(), 
            CreateMasterEditionV3CpiAccounts {
                edition: &ctx.accounts.master_edition.to_account_info(),
                mint: &ctx.accounts.nft_mint.to_account_info(),
                update_authority: &ctx.accounts.user.to_account_info(),
                mint_authority: &ctx.accounts.user.to_account_info(),
                payer: &ctx.accounts.user.to_account_info(),
                metadata: &ctx.accounts.metadata_account.to_account_info(),
                token_program: &ctx.accounts.token_program.to_account_info(),
                system_program: &ctx.accounts.system_program.to_account_info(),
                rent: Some(&ctx.accounts.rent.to_account_info())
            }, 
            CreateMasterEditionV3InstructionArgs {
                max_supply: Some(0)
            }
        ).invoke()?;

        
        Ok(())
    }

    pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
        msg!("Token mint initialized: {}", ctx.accounts.token_mint.key());

        let signer_bump = ctx.bumps.mint_authority;
        let signer_seeds = &["mint-authority".as_bytes(), &[signer_bump]];
        let signer = &[&signer_seeds[..]];
    
        // build our instruction using the accounts and data structs above and invoke it
        CreateMetadataAccountV3Cpi::new(
            &ctx.accounts.metadata_program.to_account_info(),
            CreateMetadataAccountV3CpiAccounts {
                metadata: &ctx.accounts.metadata_account.to_account_info(),
                mint: &ctx.accounts.token_mint.to_account_info(),
                mint_authority: &ctx.accounts.mint_authority.to_account_info(),
                payer: &ctx.accounts.payer.to_account_info(),
                update_authority: (&ctx.accounts.mint_authority.to_account_info(), false),
                system_program: &ctx.accounts.system_program.to_account_info(),
                rent: Some(&ctx.accounts.rent.to_account_info())
            },
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name: "Reward Token".to_string(),
                    symbol: "RWT".to_string(),
                    uri: "".to_string(),
                    seller_fee_basis_points: 0,
                    creators: None,
                    collection: None,
                    uses: None
                },
                is_mutable: true,
                collection_details: None
            }
        ).invoke_signed(signer)?;
    
        Ok(())
    }

    pub fn delegate_nft(ctx: Context<DelegateNFT>) -> Result<()> {
        let approve_nft = ctx.accounts.approve_nft_ctx();
        approve(approve_nft, 1)?;
        msg!("Approve successful");

        Ok(())
    }

    pub fn undelegate_nft(ctx: Context<DelegateNFT>) -> Result<()> {
        let revoke_nft = ctx.accounts.revoke_nft_ctx();
        revoke(revoke_nft)?;
        msg!("Revoke successful");

        Ok(())
    }
    
    pub fn stake_nft(ctx: Context<StakeNFT>) -> Result<()> {
        // TODO: Create NFT Record with true/false to determine if currently staked
        ctx.accounts.nft_record.staked_at = Clock::get().unwrap().unix_timestamp;

        let signer_bump = ctx.bumps.staking_authority;
        let signer_seeds = &["staking_Authority".as_bytes(), &[signer_bump]];
        let signer = &[&signer_seeds[..]];
    
        FreezeDelegatedAccountCpi::new(
            &ctx.accounts.metadata_program.to_account_info(),
            FreezeDelegatedAccountCpiAccounts {
                delegate: &ctx.accounts.staking_authority.to_account_info(),
                token_account: &ctx.accounts.user_token_account.to_account_info(),
                edition: &ctx.accounts.master_edition.to_account_info(),
                mint: &ctx.accounts.nft_mint.to_account_info(),
                token_program: &ctx.accounts.token_program.to_account_info()
            }
        ).invoke_signed(signer)?;
    
        Ok(())
    }

    pub fn unstake_nft(ctx: Context<UnstakeNFT>) -> Result<()> {
        // unfreeze NFT
        let signer_bump = ctx.bumps.staking_authority;
        let signer_seeds = &["staking_Authority".as_bytes(), &[signer_bump]];
        let signer = &[&signer_seeds[..]];
    
        ThawDelegatedAccountCpi::new(
            &ctx.accounts.metadata_program.to_account_info(),
            ThawDelegatedAccountCpiAccounts {
                delegate: &ctx.accounts.staking_authority.to_account_info(),
                token_account: &ctx.accounts.user_token_account.to_account_info(),
                edition: &ctx.accounts.master_edition.to_account_info(),
                mint: &ctx.accounts.nft_mint.to_account_info(),
                token_program: &ctx.accounts.token_program.to_account_info()
            }
        ).invoke_signed(signer)?;

        Ok(())
    }
    
    pub fn send_rewards(ctx: Context<AirdropRewards>) -> Result<()> {

        // current time - time staked at
        let reward_amount = Clock::get().unwrap().unix_timestamp.checked_sub(ctx.accounts.nft_record.staked_at).unwrap() as u64;

        let mint_bump = ctx.bumps.mint_authority;
        let mint_seeds = &["mint-authority".as_bytes(), &[mint_bump]];
        let signer = &[&mint_seeds[..]];
    
        msg!("Sending {} reward tokens..", reward_amount);
        let mint_to_cex = ctx.accounts.mint_to_ctx().with_signer(signer);
        mint_to(mint_to_cex, reward_amount)?;
    
        msg!("Transaction complete!");
    
        Ok(())
    }

    pub fn close_record(_ctx: Context<Close>) -> Result<()> {
        msg!("NFT Record Closed!");
        Ok(())
    }

}


#[derive(Accounts)]
pub struct CreateNFT<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        mint::decimals = 0,
        mint::authority = user,
        mint::freeze_authority = user
    )]
    pub nft_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = user,
        associated_token::mint = nft_mint,
        associated_token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    /// CHECK: safe metadata account
    #[account(
        mut,
        seeds = [b"metadata", metadata_program.key().as_ref(), nft_mint.key().as_ref()],
        bump,
        seeds::program = metadata_program.key()
    )]
    pub metadata_account: AccountInfo<'info>,
    /// CHECK: safe master edition account
    #[account(
        mut,
        seeds = [b"metadata", metadata_program.key().as_ref(), nft_mint.key().as_ref(), b"edition"],
        bump,
        seeds::program = metadata_program.key()
    )]
    pub master_edition: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: safe because we verify this is the metadata program
    #[account(
        constraint = metadata_program.key() == METADATA_PROGRAM_ID
    )]
    pub metadata_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}

impl<'info> CreateNFT <'info> {
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.nft_mint.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.user.to_account_info()
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init, 
        mint::authority = mint_authority,
        mint::decimals = 0, 
        payer = payer
    )]
    pub token_mint: Account<'info, Mint>,
    /// CHECK: using as signer
    #[account(seeds = ["mint-authority".as_bytes()], bump)]
    pub mint_authority: AccountInfo<'info>,
    /// CHECK: we verify this is associated with the metadata program
    #[account(
        mut,
        seeds = [b"metadata", metadata_program.key().as_ref(), token_mint.key().as_ref()],
        bump,
        seeds::program = metadata_program.key()
    )]
    pub metadata_account: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: safe because we verify this is the metadata program
    #[account(
        constraint = metadata_program.key() == METADATA_PROGRAM_ID
    )]
    pub metadata_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct DelegateNFT<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: Only being used as program signer
    #[account(
        seeds = ["staking_Authority".as_bytes()],
        bump
    )]
    pub staking_authority: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
        constraint = user_token_account.amount == 1 @ StakingError::TokenAccountEmpty
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(
        mint::decimals = 0,       
    )]
    pub nft_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

impl <'info> DelegateNFT<'info> {
    pub fn approve_nft_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Approve {
            to: self.user_token_account.to_account_info(),
            delegate: self.staking_authority.to_account_info(),
            authority: self.user.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn revoke_nft_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Revoke {
            source: self.user_token_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        close = user
    )]
    pub nft_record: Account<'info, NFTRecord>,
}

#[derive(Accounts)]
pub struct StakeNFT<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mint::decimals = 0,       
    )]
    pub nft_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
        constraint = user_token_account.amount == 1 @ StakingError::TokenAccountEmpty
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = user,
        seeds = ["nft_record".as_bytes(), user.key().as_ref(), nft_mint.key().as_ref()],
        bump,
        space = 8 + 8
    )]
    pub nft_record: Account<'info, NFTRecord>,
    /// CHECK: Only being used as program signer
    #[account(
        mut,
        seeds = ["staking_Authority".as_bytes()],
        bump
    )]
    pub staking_authority: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"metadata", metadata_program.key().as_ref(), nft_mint.key().as_ref(), b"edition"],
        bump,
        seeds::program = metadata_program.key(),
        constraint = master_edition.max_supply == Some(0) @ StakingError::TokenNotNFT
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,
    /// CHECK: safe because we verify this is the metadata program
    #[account(
        constraint = metadata_program.key() == METADATA_PROGRAM_ID
    )]
    pub metadata_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct UnstakeNFT<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mint::decimals = 0,       
    )]
    pub nft_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
        constraint = user_token_account.amount == 1 @ StakingError::TokenAccountEmpty
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = ["nft_record".as_bytes(), user.key().as_ref(), nft_mint.key().as_ref()],
        bump,
    )]
    pub nft_record: Account<'info, NFTRecord>,
    /// CHECK: Only being used as program signer
    #[account(
        mut,
        seeds = ["staking_Authority".as_bytes()],
        bump
    )]
    pub staking_authority: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"metadata", metadata_program.key().as_ref(), nft_mint.key().as_ref(), b"edition"],
        bump,
        seeds::program = metadata_program.key(),
        constraint = master_edition.max_supply == Some(0) @ StakingError::TokenNotNFT
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,
    /// CHECK: safe because we verify this is the metadata program
    #[account(
        constraint = metadata_program.key() == METADATA_PROGRAM_ID
    )]
    pub metadata_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>
}

#[account]
pub struct NFTRecord {
    pub staked_at: i64
}

#[derive(Accounts)]
pub struct AirdropRewards<'info> {
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    /// CHECK: using as signer
    #[account(mut, seeds = ["mint-authority".as_bytes()], bump)]
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut, 
        token::mint = token_mint,
        token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(
        mint::decimals = 0,       
    )]
    pub nft_mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = ["nft_record".as_bytes(), user.key().as_ref(), nft_mint.key().as_ref()],
        bump,
    )]
    pub nft_record: Account<'info, NFTRecord>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl <'info> AirdropRewards<'info> {
    pub fn mint_to_ctx(&self) -> CpiContext<'_,'_,'_, 'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.token_mint.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.mint_authority.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[error_code]
pub enum StakingError {
    #[msg("This token has a supply greater than 1 so it is not an NFT")]
    TokenNotNFT,
    #[msg("Associated token accounts holdes no tokens")]
    TokenAccountEmpty
}
