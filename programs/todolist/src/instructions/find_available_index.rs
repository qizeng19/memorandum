use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

pub fn handle_find_available_index(ctx: Context<FindAvailableIndex>) -> Result<u8> {
    let user_state = &ctx.accounts.user_state;
    let index_array = &user_state.index_array;

    let available_index = index_array
        .iter()
        .position(|&index| index == 0)
        .ok_or(ErrorCode::NoAvailableIndex)?;

    Ok(available_index as u8)   
}

#[derive(Accounts)]
pub struct FindAvailableIndex<'info> {
    #[account(
        seeds = [b"user", user.key().as_ref()],
        bump = user_state.bump
    )]
    pub user_state: Account<'info, UserState>,
    pub user: Signer<'info>,
}
