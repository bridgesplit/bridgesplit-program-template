use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

pub mod errors;
mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

declare_id!("6QXj6hJwibgSTUCMiphcoQs1sBp66g8L3WXQCgGugG2h");

#[program]
pub mod example {
    use super::*;

    pub fn example(ctx: Context<ExampleInstruction>) -> ProgramResult {
        instructions::ix::handler(ctx)
    }
}
