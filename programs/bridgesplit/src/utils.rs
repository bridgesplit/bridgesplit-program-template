use anchor_lang::prelude::*;

use crate::errors::*;

#[inline(always)]
pub fn lamport_transfer<'info>(
    src: AccountInfo<'info>,
    dest: AccountInfo<'info>,
    amount: u64,
) -> ProgramResult {
    **dest.lamports.borrow_mut() = dest.lamports()
        .checked_add(amount)
        .ok_or(ErrorCode::ExampleError)?;
    **src.lamports.borrow_mut() = src.lamports()
        .checked_sub(amount)
        .ok_or(ErrorCode::ExampleError)?;
    Ok(())
}