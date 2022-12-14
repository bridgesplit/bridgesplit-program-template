use anchor_lang::prelude::*;
use num_enum::IntoPrimitive;

// Enum to encode the state of a fractional sale
#[derive(Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
pub enum SaleState {
    OPEN,
    CANCELED,
    SOLD,
    CLOSED,
}

// PDA for a fractional sale
#[account()]
pub struct SalesVault {
    pub seller: Pubkey,         // User selling the fractions
    pub fractions_mint: Pubkey, // Mint of fractions for sale
    pub fractions: u64,         // Number of fractions for sale
    pub nft_mint: Pubkey,       // Mint of NFT being fractionalized
    pub payment_mint: Pubkey,   // Mint that must be paid to win fractions
    pub price: u64,             // Amount that must be paid for fractions
    pub state: u8,              // Current SaleState
}
