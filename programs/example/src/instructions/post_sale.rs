use anchor_lang::Key;
use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};

use crate::state::*;

#[derive(Accounts)]
#[instruction()]
pub struct PostSale<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        init,
        seeds = [VAULT_SEED.as_ref(),
        fractions_mint.key().as_ref()],
        bump,
        payer = seller,
        space = 8 + std::mem::size_of::<SalesVault>(),
    )]
    pub sales_vault: Box<Account<'info, SalesVault>>,

    
    #[account(mut)]
    pub bridgesplit_vault: AccountInfo<'info>,

    #[account(mut)]
    pub nft_mint: Box<Account<'info, Mint>>,
    
    #[account(
        mut,
        constraint = seller_nft_account.mint == nft_mint.key(),
        constraint = seller_nft_account.owner == seller.key(),
    )]
    pub seller_nft_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = bs_vault_nft_account.mint == nft_mint.key(),
        constraint = bs_vault_nft_account.owner == bridgesplit_vault.key(),
    )]
    pub bs_vault_nft_account: Box<Account<'info, TokenAccount>>,
    
    pub fractions_mint: ,

    #[account(
        mut,
        constraint = seller_fractions_account.mint == fractions_mint.key(),
        constraint = seller_fractions_account.owner == seller.key(),
    )]
    pub seller_fractions_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = sales_vault_fractions_account.mint == fractions_mint.key(),
        constraint = sales_vault_fractions_account.owner == sales_vault.key(),
    )]
    pub sales_vault_fractions_account: Box<Account<'info, TokenAccount>>,
}

impl<'info> PostSale<'info> {}

pub fn handler(ctx: Context<PostSale>) -> ProgramResult {
    // Init sales vault act

    // Fxnlize nft

    // transfer fractions to sales vault


    Ok(())
}
