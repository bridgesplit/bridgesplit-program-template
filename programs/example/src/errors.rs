use anchor_lang::prelude::*;

#[error_code]
pub enum SpecificErrorCode {
    #[msg("Can't sell more than the whole NFT")]
    OversoldNFTError,
}
