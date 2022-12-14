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

    pub fn buyout(ctx: Context<Buyout>) -> ProgramResult {
        instructions::buyout::handler(ctx)
    }

    pub fn cancel_sale(ctx: Context<CancelSale>) -> ProgramResult {
        instructions::cancel_sale::handler(ctx)
    }

    pub fn claim_buyout(ctx: Context<ClaimBuyout>) -> ProgramResult {
        instructions::claim_buyout::handler(ctx)
    }
}
