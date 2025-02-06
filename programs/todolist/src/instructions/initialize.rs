use anchor_lang::prelude::*;
use crate::state::*;
use crate::constant::*;
use crate::error::ErrorCode;
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init, 
        payer = user, 
        space =  8+ UserState::INIT_SPACE, 
        seeds = [b"user", user.key().as_ref()], 
        bump
    )]
    pub user_state: Account<'info, UserState>,
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize(ctx: Context<Initialize>, max_length: u8) -> Result<()> {
    let user_state = &mut ctx.accounts.user_state;

    if max_length > MAX_LENGTH {
        // return Err(ErrorCode::InvalidMaxLength)?;// 也可
        return Err(ErrorCode::InvalidMaxLength.into());
    }
    user_state.user = ctx.accounts.user.key();
    user_state.index_array = vec![0; max_length as usize];
    user_state.bump = ctx.bumps.user_state;
    Ok(())
}
