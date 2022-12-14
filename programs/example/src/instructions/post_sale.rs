use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use anchor_spl::associated_token::Create;
use anchor_spl::token;
use anchor_spl::{associated_token, associated_token::AssociatedToken, token::*};
use vault::cpi::accounts::Fractionalize;
use vault::program::Vault;

use crate::state::*;

// Called when a user would like to sell a portion of an NFT
#[derive(Accounts)]
#[instruction()]
pub struct PostSale<'info> {
    // User selling the nft
    #[account(mut)]
    pub seller: Signer<'info>,

    // PDA that will correspond to this sale
    #[account(
        init,                                          // Will be initialized in this instruction
        seeds = [VAULT_SEED,
        fractions_mint.key().as_ref()],                // Fractions mint are seeded with a nonce, so this won't have collisions
        bump,
        payer = seller,                                // User is responsible to fund PDA
        space = 8 + std::mem::size_of::<SalesVault>(), // Getting size of PDA from state struct
    )]
    pub sales_vault: Box<Account<'info, SalesVault>>,

    // Mint that user would like to be paid in
    #[account(mut)]
    pub payment_mint: Box<Account<'info, Mint>>,

    // Nft that user would like to partially sell
    #[account(
        mut,
        constraint = nft_mint.supply == 1,   // Must have supply of 1
        constraint = nft_mint.decimals == 0, // Must have no decimals... These are definition of NFT
    )]
    pub nft_mint: Box<Account<'info, Mint>>,

    // Token Account of users NFT
    #[account(
        mut,
        constraint = seller_nft_account.mint == nft_mint.key(),
        constraint = seller_nft_account.owner == seller.key(),
    )]
    pub seller_nft_account: Box<Account<'info, TokenAccount>>,

    // Bridgesplit PDA that will hold NFT after fractionalization
    /// CHECK: Checks in Bridgesplit
    #[account(mut)]
    pub bridgesplit_vault: AccountInfo<'info>,

    // Bridgesplit Program to interact with
    pub bridgesplit_program: Program<'info, Vault>,

    // Token Account for NFT belonging to Bridgesplit PDA
    /// CHECK: Checks in Bridgesplit
    #[account(mut)]
    pub bs_vault_nft_account: UncheckedAccount<'info>,

    // Mint of fractions that will correspond to this NFT
    /// CHECK: Checks in Bridgesplit
    #[account(mut)]
    pub fractions_mint: UncheckedAccount<'info>,

    // Token Account where seller will recieve fractions that aren't for sale
    /// CHECK: Checks in Bridgesplit
    #[account(mut)]
    pub seller_fractions_account: UncheckedAccount<'info>,

    // Token Account belonging to SalesVault PDA where fractions for sale will be held
    /// CHECK: Checks in Associated Token Program
    #[account(mut)]
    pub sales_vault_fractions_account: UncheckedAccount<'info>,

    // Required for creation of fractional mint
    /// CHECK: Checks done in Bridgesplit
    pub mpl_token_metadata: UncheckedAccount<'info>,

    // Required for creation of fractional mint
    /// CHECK: Checks in Bridgesplit
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    // Associated token program to Create ATAs
    pub associated_token_program: Program<'info, AssociatedToken>,

    // System Program to allow creation of PDA
    pub system_program: Program<'info, System>,

    // Token Program for the exchange of tokens
    pub token_program: Program<'info, Token>,

    // Rent Sysvar for the creation of PDAs
    pub rent: Sysvar<'info, Rent>,

    // Clock Sysvar required for fractionalization
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> PostSale<'info> {
    // Creating CPI context for Fractionalize NFT instruction
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

    // Getting CPI Context for creating Sales Vault fractions ATA
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

    // Getting CPI Context to send fractions for sale to Sales Vault PDA
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
    price: u64,        // Cost to buy for sale fractions
    nonce: Pubkey,     // Nonce used to fractionalize NFT
    sold_shares: u64,  // Number of shares to sell
    total_shares: u64, // Numbre of shares to split NFT into
    uri: String,       // URI for fractions metadata
    name: String,      // Name for fractions metadata
    symbol: String,    // Symbol for fractions metadata
) -> ProgramResult {
    // Initialize the SalesVault PDA
    let sales_vault = &mut ctx.accounts.sales_vault; // Obtaining mutable reference
                                                     // Setting the initial state of PDA
    sales_vault.seller = ctx.accounts.seller.key();
    sales_vault.fractions_mint = ctx.accounts.fractions_mint.key();
    sales_vault.fractions = sold_shares;
    sales_vault.nft_mint = ctx.accounts.nft_mint.key();
    sales_vault.payment_mint = ctx.accounts.payment_mint.key();
    sales_vault.price = price;
    sales_vault.state = SaleState::OPEN.into();

    // Fxnlize nft
    // CPI into Bridgesplit fractionalize instruction
    vault::cpi::fractionalize(
        ctx.accounts.fractionalize(),
        nonce,
        total_shares,
        uri,
        name,
        symbol,
    )?;

    // Create sales_vault ATA
    // CPI into Associated Token Program
    associated_token::create(ctx.accounts.create_ata())?;

    // transfer fractions to sales vault
    // CPI into token program to transfer fractions
    token::transfer(ctx.accounts.transfer_fractions(), sold_shares)?;

    Ok(()) // Return from instruction
}
