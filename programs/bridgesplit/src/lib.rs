use anchor_lang::prelude::*;

pub mod errors;
mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

declare_id!("6QXj6hJwibgSTUCMiphcoQs1sBp66g8L3WXQCgGugG2h");

#[program]
pub mod example {
    use super::*;

    pub fn example(
        ctx: Context<ExampleInstruction>, 
        _example_bump: u8, 
    ) -> ProgramResult {
        instructions::example_instruction::handler(ctx, _example_bump)
    }
}