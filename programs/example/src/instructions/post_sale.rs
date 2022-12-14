use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use anchor_spl::associated_token::Create;
use anchor_spl::token;
use anchor_spl::{associated_token, associated_token::AssociatedToken, token::*};
use vault::cpi::accounts::Fractionalize;

use crate::state::*;

#[derive(Accounts)]
#[instruction()]
pub struct PostSale<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        init,
        seeds = [VAULT_SEED,
        fractions_mint.key().as_ref()],
        bump,
        payer = seller,
        space = 8 + std::mem::size_of::<SalesVault>(),
    )]
    pub sales_vault: Box<Account<'info, SalesVault>>,

    #[account(mut)]
    pub payment_mint: Box<Account<'info, Mint>>,

    /// CHECK: Checks in Bridgesplit
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
    /// CHECK: Checks in Bridgesplit
    #[account(mut)]
    pub bs_vault_nft_account: UncheckedAccount<'info>,
    /// CHECK: Checks in Bridgesplit
    #[account(mut)]
    pub fractions_mint: AccountInfo<'info>,
    /// CHECK: Checks in Bridgesplit
    #[account(mut)]
    pub seller_fractions_account: UncheckedAccount<'info>,
    /// CHECK: Checks in Associated Token Program
    #[account(mut)]
    pub sales_vault_fractions_account: UncheckedAccount<'info>,
    /// CHECK: Checks done in Bridgesplit
    pub bridgesplit_program: UncheckedAccount<'info>,
    /// CHECK: Checks done in Bridgesplit
    pub mpl_token_metadata: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: Checks in Bridgesplit
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> PostSale<'info> {
    fn fractionalize(&self) -> CpiContext<'_, '_, '_, 'info, Fractionalize<'info>> {
        let cpi_accounts = Fractionalize {
            payer_account: self.seller.to_account_info(),
            initializer_account: self.seller.to_account_info(),
            fractions_mint: self.fractions_mint.to_account_info(),
            fractions_token_account: self.seller_fractions_account.to_account_info(),
            nft_mint: self.nft_mint.to_account_info(),
            initializer_nft_account: self.seller_nft_account.to_account_info(),
            program_nft_account: self.bs_vault_nft_account.to_account_info(),
            vault_account: self.bridgesplit_vault.to_account_info(),
            associated_token_program: self.associated_token_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
            mpl_token_metadata: self.mpl_token_metadata.to_account_info(),
            metadata_account: self.metadata_account.to_account_info(),
            rent: self.rent.to_account_info(),
            clock: self.clock.to_account_info(),
        };
        CpiContext::new(self.bridgesplit_program.to_account_info(), cpi_accounts)
    }

    fn create_ata(&self) -> CpiContext<'_, '_, '_, 'info, Create<'info>> {
        let cpi_accounts = Create {
            payer: self.seller.to_account_info(),
            associated_token: self.sales_vault_fractions_account.to_account_info(),
            authority: self.sales_vault.to_account_info(),
            mint: self.fractions_mint.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        CpiContext::new(
            self.associated_token_program.to_account_info(),
            cpi_accounts,
        )
    }

    fn transfer_fractions(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.seller_fractions_account.to_account_info(),
            to: self.sales_vault_fractions_account.to_account_info(),
            authority: self.seller.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

#[allow(clippy::too_many_arguments)]
pub fn handler(
    ctx: Context<PostSale>,
    price: u64,
    nonce: Pubkey,
    sold_shares: u64,
    total_shares: u64,
    uri: String,
    name: String,
    symbol: String,
) -> ProgramResult {
    // Init sales vault act
    let sales_vault = &mut ctx.accounts.sales_vault;
    sales_vault.seller = ctx.accounts.seller.key();
    sales_vault.fractions_mint = ctx.accounts.fractions_mint.key();
    sales_vault.fractions = sold_shares;
    sales_vault.nft_mint = ctx.accounts.nft_mint.key();
    sales_vault.payment_mint = ctx.accounts.payment_mint.key();
    sales_vault.price = price;
    sales_vault.state = SaleState::OPEN.into();

    // Fxnlize nft
    vault::cpi::fractionalize(
        ctx.accounts.fractionalize(),
        nonce,
        total_shares,
        uri,
        name,
        symbol,
    )?;

    // Create sales_vault ATA
    associated_token::create(ctx.accounts.create_ata())?;

    // transfer fractions to sales vault
    token::transfer(ctx.accounts.transfer_fractions(), sold_shares)?;

    Ok(())
}
