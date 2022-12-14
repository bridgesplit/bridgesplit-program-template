use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use anchor_spl::token;
use anchor_spl::token::*;

use crate::state::*;
use crate::utils::get_bump_in_seed_form;

#[derive(Accounts)]
#[instruction()]
pub struct CancelSale<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        constraint = sales_vault.state == SaleState::OPEN.into(),
        seeds = [VAULT_SEED,
        fractions_mint.key().as_ref()],
        bump
    )]
    pub sales_vault: Box<Account<'info, SalesVault>>,

    #[account(
        mut,
        constraint = fractions_mint.key() == sales_vault.fractions_mint,
    )]
    pub fractions_mint: Box<Account<'info, Mint>>,

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
    pub token_program: Program<'info, Token>,
}

impl<'info> CancelSale<'info> {
    fn return_fractions(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.sales_vault_fractions_account.to_account_info(),
            to: self.seller_fractions_account.to_account_info(),
            authority: self.sales_vault.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

pub fn handler(ctx: Context<CancelSale>) -> ProgramResult {
    // Return Fxns
    let vault_seeds = &[
        VAULT_SEED,
        &ctx.accounts.fractions_mint.key().to_bytes(),
        &get_bump_in_seed_form(ctx.bumps.get("sales_vault").unwrap()),
    ];
    let vault_signer = &[&vault_seeds[..]];
    token::transfer(
        ctx.accounts.return_fractions().with_signer(vault_signer),
        ctx.accounts.sales_vault.fractions,
    )?;

    // Mark as cancelled
    let sales_vault = &mut ctx.accounts.sales_vault;
    sales_vault.state = SaleState::SOLD.into();

    Ok(())
}
