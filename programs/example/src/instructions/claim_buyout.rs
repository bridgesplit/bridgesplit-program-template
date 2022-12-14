use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use anchor_spl::token;
use anchor_spl::token::*;

use crate::state::*;
use crate::utils::get_bump_in_seed_form;

// Called when a seller wants to claim their payment for sold fractions
#[derive(Accounts)]
#[instruction()]
pub struct ClaimBuyout<'info> {
    // User who sold their fractions
    #[account(mut)]
    pub seller: Signer<'info>,

    // PDA for the sale
    #[account(
        mut,
        constraint = sales_vault.state == SaleState::SOLD.into(), // Cannot claim payment if it isn't sold
        constraint = sales_vault.fractions_mint == fractions_mint.key(),
        constraint = sales_vault.seller == seller.key(), // Must be the user who started the sale
        seeds = [VAULT_SEED,
        fractions_mint.key().as_ref()],
        bump
    )]
    pub sales_vault: Box<Account<'info, SalesVault>>,

    // Mint of the fractions Sold (Needed to find PDA)
    #[account(
        mut,
        constraint = fractions_mint.key() == sales_vault.fractions_mint,
    )]
    pub fractions_mint: Box<Account<'info, Mint>>,

    // Mint that the user is being paid for the sale
    #[account(
        mut,
        constraint = payment_mint.key() == sales_vault.payment_mint,
    )]
    pub payment_mint: Box<Account<'info, Mint>>,

    // Users token account to recieve payment
    #[account(
        mut,
        constraint = seller_payment_account.mint == payment_mint.key(),
        constraint = seller_payment_account.owner == seller.key()
    )]
    pub seller_payment_account: Box<Account<'info, TokenAccount>>,

    // PDA's token account where payment is being held
    #[account(
        mut,
        constraint = sales_vault_payment_account.mint == payment_mint.key(),
        constraint = sales_vault_payment_account.owner == sales_vault.key(),
        constraint = sales_vault_payment_account.amount >= sales_vault.price, // Must have enough funds
    )]
    pub sales_vault_payment_account: Box<Account<'info, TokenAccount>>,

    // Token Program to allow transfer of tokens
    pub token_program: Program<'info, Token>,
}

impl<'info> ClaimBuyout<'info> {
    // Getting CPI context for transfer of tokens
    fn transfer_sale(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.sales_vault_payment_account.to_account_info(),
            to: self.seller_payment_account.to_account_info(),
            authority: self.sales_vault.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

pub fn handler(ctx: Context<ClaimBuyout>) -> ProgramResult {
    // Pass funds on to the seller
    // Obtaining PDA signer seeds
    let vault_seeds = &[
        VAULT_SEED,
        &ctx.accounts.fractions_mint.key().to_bytes(),
        &get_bump_in_seed_form(ctx.bumps.get("sales_vault").unwrap()),
    ];
    let vault_signer = &[&vault_seeds[..]];
    // CPI into token program
    token::transfer(
        ctx.accounts.transfer_sale().with_signer(vault_signer), // CPI Context with signer seeds signing
        ctx.accounts.sales_vault.price,                         // Argument for transfer
    )?;

    // Mark as closed
    let sales_vault = &mut ctx.accounts.sales_vault; // Getting mutable copy of PDA
    sales_vault.state = SaleState::CLOSED.into(); // Changing to closed

    Ok(()) // Returning from instruction
}
