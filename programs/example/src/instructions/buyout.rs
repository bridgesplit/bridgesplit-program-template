use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use anchor_spl::token::{self, *};

use crate::{state::*, utils::get_bump_in_seed_form};

// Instruction called when someone wants to buyout fractions that are for sale
#[derive(Accounts)]
#[instruction()]
pub struct Buyout<'info> {
    // User who is buying the fractions
    #[account(mut)]
    pub buyer: Signer<'info>,

    // Program Derived Account corresponding to this sale of fractions
    #[account(
        mut,
        constraint = sales_vault.state == SaleState::OPEN.into(),
        seeds = [VAULT_SEED,
        fractions_mint.key().as_ref()],
        bump
    )]
    pub sales_vault: Box<Account<'info, SalesVault>>,

    // Mint of fractions for sale
    #[account(
        mut,
        constraint = fractions_mint.key() == sales_vault.fractions_mint,
    )]
    pub fractions_mint: Box<Account<'info, Mint>>,

    // Token Account for the buyer's fractions
    #[account(
        mut,
        constraint = buyer_fractions_account.mint == fractions_mint.key(),
        constraint = buyer_fractions_account.owner == buyer.key(),
    )]
    pub buyer_fractions_account: Box<Account<'info, TokenAccount>>,

    // Token Account for PDA's fractions
    #[account(
        mut,
        constraint = sales_vault_fractions_account.mint == fractions_mint.key(),
        constraint = sales_vault_fractions_account.owner == sales_vault.key(),
    )]
    pub sales_vault_fractions_account: Box<Account<'info, TokenAccount>>,

    // Mint that must be payed for this sale
    #[account(
        mut,
        constraint = payment_mint.key() == sales_vault.payment_mint,
    )]
    pub payment_mint: Box<Account<'info, Mint>>,

    // Buyer's token account for payment_mint
    #[account(
        mut,
        constraint = buyer_payment_account.mint == payment_mint.key(),
        constraint = buyer_payment_account.owner == buyer.key(),
        constraint = buyer_payment_account.amount >= sales_vault.price,
    )]
    pub buyer_payment_account: Box<Account<'info, TokenAccount>>,

    // PDA's token account for payment mint, to hold payment
    #[account(
        mut,
        constraint = sales_vault_payment_account.mint == payment_mint.key(),
        constraint = sales_vault_payment_account.owner == sales_vault.key(),
    )]
    pub sales_vault_payment_account: Box<Account<'info, TokenAccount>>,

    // Token Program to allow the transfer of tokens
    pub token_program: Program<'info, Token>,
}

impl<'info> Buyout<'info> {
    // Function to create CPI Context for transfering fractions from Vault to Buyer
    fn transfer_fractions(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.sales_vault_fractions_account.to_account_info(),
            to: self.buyer_fractions_account.to_account_info(),
            authority: self.sales_vault.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    // Function to create CPI Context for transfering payment from Buyer to Vaultß
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
    // Send the User's Payment to the vault
    // Calling Token Program's transfer instruction
    token::transfer(
        ctx.accounts.transfer_payment(), // Getting Context for instruction
        ctx.accounts.sales_vault.price,  // Argument for instruction
    )?;

    // Send fractions to user
    // Creating signer seeds for vault PDA
    let vault_seeds = &[
        VAULT_SEED,                                    // From seeds used for creating account
        &ctx.accounts.fractions_mint.key().to_bytes(), // From seeds used for creating account
        &get_bump_in_seed_form(ctx.bumps.get("sales_vault").unwrap()), // From bump used to find account
    ];
    let vault_signer = &[&vault_seeds[..]];
    // Calling Token Program's transfer instruction
    token::transfer(
        ctx.accounts.transfer_fractions().with_signer(vault_signer), // Getting Context for instruction and Using signer seeds to sign
        ctx.accounts.sales_vault.fractions,                          // Argument for Instruction
    )?;

    // Change vault state
    let sales_vault = &mut ctx.accounts.sales_vault; // Obtaining mutable copy of PDA
    sales_vault.state = SaleState::SOLD.into(); // Changing state field of PDA

    Ok(()) // Returning from Instruction
}
