use anchor_lang::prelude::*;

#[error_code]
pub enum SpecificErrorCode {
    #[msg("Example Error")]
    ExampleError,
}
