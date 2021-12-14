use anchor_lang::error;

#[error]
pub enum ErrorCode {
    #[msg("Example Error")]
    ExampleError,
}