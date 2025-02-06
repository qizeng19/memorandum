use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(index: u8)]
pub struct Delete<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_state.bump
    )]
    pub user_state: Account<'info, UserState>,

    #[account(
        mut,
        seeds = [b"list_item", user.key().as_ref(), &index.to_le_bytes()],
        bump = list_item.bump,
        close = user
    )]      
    pub list_item: Account<'info, ListItem>,

    pub system_program: Program<'info, System>,
}

pub fn handle_delete(ctx: Context<Delete>, index: u8) -> Result<()> {
    let user_state = &mut ctx.accounts.user_state;
    let index_array = &mut user_state.index_array;
    index_array[index as usize] = 0;
    Ok(())
}
