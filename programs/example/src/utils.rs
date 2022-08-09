use anchor_lang::prelude::*;

use crate::errors::SpecificErrorCode;

#[inline(always)]
pub fn lamport_transfer<'info>(
    src: AccountInfo<'info>,
    dest: AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    **dest.lamports.borrow_mut() = dest
        .lamports()
        .checked_add(amount)
        .ok_or(SpecificErrorCode::ExampleError)?;
    **src.lamports.borrow_mut() = src
        .lamports()
        .checked_sub(amount)
        .ok_or(SpecificErrorCode::ExampleError)?;
    Ok(())
}
