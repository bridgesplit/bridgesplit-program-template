use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

pub mod errors;
mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

declare_id!("AE748h33zGqpyvoxAyGBKD1kgNH8u4TzxZHwsVkMxosk");

#[program]
pub mod example {

    use super::*;

    /// Fractionalizes User's NFT, and creates a sale for a portion of the fractions.
    /// Returns the result of the PostSale Instruction.
    ///
    /// # Arguments
    ///
    /// * `ctx` - PostSale accounts context
    /// * `price` - The amount that must be paid to recieve fractions
    /// * `nonce` - Nonce used to create Fractions Mint and Bridgesplit Vault
    /// * `sold_shares` - Number of shares that will be put up for sale
    /// * `total_shares` - Number of shares to fractionalize NFT into
    /// * `uri` - URI for Fractional Mint metadata
    /// * `name` - Name for Fractional Mint
    /// * `symbol` - Symbol for Fractional Mint
    #[allow(clippy::too_many_arguments)]
    pub fn post_sale(
        ctx: Context<PostSale>,
        price: u64,
        nonce: Pubkey,
        sold_shares: u64,
        total_shares: u64,
        uri: String,
        name: String,
        symbol: String,
    ) -> ProgramResult {
        instructions::post_sale::handler(
            ctx,
            price,
            nonce,
            sold_shares,
            total_shares,
            uri,
            name,
            symbol,
        )
    }

    /// Buys fractions that are for sale.
    /// Returns the result of the Buyout Instruction.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Buyout accounts context
    pub fn buyout(ctx: Context<Buyout>) -> ProgramResult {
        instructions::buyout::handler(ctx)
    }

    /// Cancels the sale of NFT fractions.
    /// Returns the result of the CancelSale Instruction.
    ///
    /// # Arguments
    ///
    /// * `ctx` - CancelSale accounts context
    pub fn cancel_sale(ctx: Context<CancelSale>) -> ProgramResult {
        instructions::cancel_sale::handler(ctx)
    }

    /// Allows the seller of fractions to claim their payout for selling fractions.
    /// Returns the result of the ClaimBuyout Instruction.
    ///
    /// # Arguments
    ///
    /// * `ctx` - ClaimBuyout accounts context
    pub fn claim_buyout(ctx: Context<ClaimBuyout>) -> ProgramResult {
        instructions::claim_buyout::handler(ctx)
    }
}
