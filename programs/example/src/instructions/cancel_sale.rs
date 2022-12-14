use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use anchor_spl::token;
use anchor_spl::token::*;

use crate::state::*;
use crate::utils::get_bump_in_seed_form;

// Instruction called when someone wants to cancel their sale and reclaim their fractionsß
#[derive(Accounts)]
#[instruction()]
pub struct CancelSale<'info> {
    // User who is cancelling thier sale
    #[account(mut)]
    pub seller: Signer<'info>,

    // Sale PDA that they are cancelling
    #[account(
        mut,
        constraint = sales_vault.state == SaleState::OPEN.into(), // Ensuring it has not already been sold or cancelled
        constraint = sales_vault.seller == seller.key(), // Must be the user who started the sale
        seeds = [VAULT_SEED,
        fractions_mint.key().as_ref()],
        bump
    )]
    pub sales_vault: Box<Account<'info, SalesVault>>,

    // Mint of the fractions that were for sale
    #[account(
        mut,
        constraint = fractions_mint.key() == sales_vault.fractions_mint,
    )]
    pub fractions_mint: Box<Account<'info, Mint>>,

    // Users token account for these fractions
    #[account(
        mut,
        constraint = seller_fractions_account.mint == fractions_mint.key(),
        constraint = seller_fractions_account.owner == seller.key(),
    )]
    pub seller_fractions_account: Box<Account<'info, TokenAccount>>,

    // PDA's fractions token account where fractions are stored
    #[account(
        mut,
        constraint = sales_vault_fractions_account.mint == fractions_mint.key(),
        constraint = sales_vault_fractions_account.owner == sales_vault.key(),
    )]
    pub sales_vault_fractions_account: Box<Account<'info, TokenAccount>>,

    // Token Program for transferring of tokens
    pub token_program: Program<'info, Token>,
}

impl<'info> CancelSale<'info> {
    // Creates CPI context for trade of fractions from vault to seller
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
    // Return Fractions
    // Creaing signer seeds
    // Creating signer seeds for vault PDA
    let vault_seeds = &[
        VAULT_SEED,                                    // From seeds used for creating account
        &ctx.accounts.fractions_mint.key().to_bytes(), // From seeds used for creating account
        &get_bump_in_seed_form(ctx.bumps.get("sales_vault").unwrap()), // From bump used to find account
    ];
    let vault_signer = &[&vault_seeds[..]];
    // CPI into Token Program's Transfer Instruction
    token::transfer(
        ctx.accounts.return_fractions().with_signer(vault_signer), // CPI Context with signer seeds as signer
        ctx.accounts.sales_vault.fractions,                        // Argument for transfer
    )?;

    // Mark as cancelled
    let sales_vault = &mut ctx.accounts.sales_vault; // Obtaining mutable copy of PDA
    sales_vault.state = SaleState::CANCELED.into(); // Changing state to CLOSED

    Ok(()) // Return from instruction
}
