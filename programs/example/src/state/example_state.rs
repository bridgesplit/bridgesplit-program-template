use anchor_lang::prelude::*;
use num_enum::IntoPrimitive;

#[derive(Clone, Copy, PartialEq, IntoPrimitive)]
#[repr(u8)]
pub enum SaleState {
    OPEN,
    CANCELED,
    SOLD
}

#[account()]
pub struct SalesVault {
    pub seller: Pubkey,
    pub fractions_mint: Pubkey,
    pub nft_mint: Pubkey,
    pub payment_mint: Pubkey,
    pub price: u64,
    pub state: u8
}
