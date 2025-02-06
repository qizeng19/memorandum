use anchor_lang::prelude::*;
use crate::state::*;
#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub user_state: Account<'info, UserState>,
    pub system_program: Program<'info, System>,
}

pub fn handle_add(ctx: Context<Add>, content: String) -> Result<()> {
    Ok(())
}
