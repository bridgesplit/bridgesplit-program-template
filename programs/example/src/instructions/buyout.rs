use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use anchor_spl::token::{self, *};

use crate::{state::*, utils::get_bump_in_seed_form};

#[derive(Accounts)]
#[instruction()]
pub struct Buyout<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        constraint = payment_mint.key() == sales_vault.payment_mint,
    )]
    pub payment_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        constraint = fractions_mint.key() == sales_vault.fractions_mint,
    )]
    pub fractions_mint: Box<Account<'info, Mint>>,

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
        constraint = buyer_fractions_account.mint == fractions_mint.key(),
        constraint = buyer_fractions_account.owner == buyer.key(),
    )]
    pub buyer_fractions_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = sales_vault_fractions_account.mint == fractions_mint.key(),
        constraint = sales_vault_fractions_account.owner == sales_vault.key(),
    )]
    pub sales_vault_fractions_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = buyer_payment_account.mint == payment_mint.key(),
        constraint = buyer_payment_account.owner == buyer.key(),
        constraint = buyer_payment_account.amount >= sales_vault.price,
    )]
    pub buyer_payment_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = sales_vault_payment_account.mint == payment_mint.key(),
        constraint = sales_vault_payment_account.owner == sales_vault.key(),
    )]
    pub sales_vault_payment_account: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Buyout<'info> {
    fn transfer_fractions(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.sales_vault_fractions_account.to_account_info(),
            to: self.buyer_fractions_account.to_account_info(),
            authority: self.sales_vault.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn transfer_payment(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.buyer_payment_account.to_account_info(),
            to: self.sales_vault_payment_account.to_account_info(),
            authority: self.buyer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

pub fn handler(ctx: Context<Buyout>) -> ProgramResult {
    // Send payment to vault
    token::transfer(
        ctx.accounts.transfer_payment(),
        ctx.accounts.sales_vault.price,
    )?;

    // Send fractions to user
    let vault_seeds = &[
        VAULT_SEED,
        &ctx.accounts.fractions_mint.key().to_bytes(),
        &get_bump_in_seed_form(ctx.bumps.get("sales_vault").unwrap()),
    ];
    let vault_signer = &[&vault_seeds[..]];
    token::transfer(
        ctx.accounts.transfer_fractions().with_signer(vault_signer),
        ctx.accounts.sales_vault.fractions,
    )?;

    // Change vault state
    let sales_vault = &mut ctx.accounts.sales_vault;
    sales_vault.state = SaleState::SOLD.into();

    Ok(())
}
