use anchor_lang::prelude::*;

#[account()]
pub struct ExampleAccount {
    pub initializer_account_pubkey: Pubkey,
    pub creation_time: i64,
    pub reserve: [u8; 512],
}
