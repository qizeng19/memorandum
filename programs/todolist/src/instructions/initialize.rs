use anchor_lang::prelude::*;
use crate::state::*;

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

pub fn handle_initialize(ctx: Context<Initialize>) -> Result<()> {
    let user_state = &mut ctx.accounts.user_state;
    user_state.user = ctx.accounts.user.key();
    user_state.index = 0;
    user_state.max_lenth = 10;
    Ok(())
}
