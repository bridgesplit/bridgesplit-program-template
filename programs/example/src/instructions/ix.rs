use anchor_lang::Key;
use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};

use crate::state::*;

#[derive(Accounts)]
#[instruction()]
pub struct ExampleInstruction<'info> {
    #[account(mut)]
    pub initializer_account: Signer<'info>,
    #[account(
        init,
        seeds = [EXAMPLE_SEED,
        initializer_account.key().as_ref()],
        bump,
        payer = initializer_account,
        space = 8 + std::mem::size_of::<ExampleAccount>())]
    pub example_account: Box<Account<'info, ExampleAccount>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> ExampleInstruction<'info> {}

pub fn handler(ctx: Context<ExampleInstruction>) -> ProgramResult {
    msg!("Beginning instruction ...");

    let example_account = &mut ctx.accounts.example_account;
    example_account.initializer_account_pubkey = ctx.accounts.initializer_account.key();

    example_account.creation_time = ctx.accounts.clock.unix_timestamp;

    msg!("Example complete.");

    Ok(())
}
