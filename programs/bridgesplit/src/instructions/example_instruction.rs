use anchor_lang::prelude::*;
use anchor_lang::Key;
use anchor_spl::token;
use spl_token::instruction::AuthorityType;

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
#[instruction(example_bump: u8)]
pub struct ExampleInstruction<'info> {
    #[account(mut, signer)]
    pub initializer_account: AccountInfo<'info>,
    #[account(
        init,
        seeds = [EXAMPLE_SEED.as_ref(), 
        initializer_account.key().as_ref()],
        bump = example_bump, 
        payer = initializer_account,
        space = 8 + std::mem::size_of::<ExampleAccount>())]
    pub example_account: ProgramAccount<'info, ExampleAccount>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

impl <'info> ExampleInstruction<'info> {
}

pub fn handler(
    ctx: Context<ExampleInstruction>, 
    _example_bump: u8,) -> ProgramResult {
        msg!("Beginning instruction ...");
        
        let example_account = &mut ctx.accounts.example_account;
        example_account.initializer_account_pubkey = ctx.accounts.initializer_account.key();
        
        example_account.creation_time = ctx.accounts.clock.unix_timestamp;

        msg!("Example complete.");

        Ok(())
}