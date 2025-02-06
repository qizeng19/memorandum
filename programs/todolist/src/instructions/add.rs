use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
#[derive(Accounts)]
#[instruction(_content: String, available_index: u8)]
pub struct Add<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_state.bump
    )]
    pub user_state: Account<'info, UserState>,

 
    #[account(
        init,
        payer = user,
        space = 8 + ListItem::INIT_SPACE,
        seeds = [b"list_item", user.key().as_ref(), &available_index.to_le_bytes()],
        bump 
    )]
    pub list_item: Account<'info, ListItem>,

    pub system_program: Program<'info, System>,
}

pub fn handle_add(ctx: Context<Add>, content: String, available_index: u8) -> Result<()> {
    
    let user_state = &mut ctx.accounts.user_state;
    let index_array = &mut user_state.index_array;
    let list_item = &mut ctx.accounts.list_item;

    if !index_array.contains(&0) {
        return Err(ErrorCode::NoAvailableIndex.into());
    }

    index_array[available_index as usize] = 1;

    list_item.user = ctx.accounts.user.key();
    list_item.index = available_index;
    list_item.content = content;
    list_item.is_completed = false;

    Ok(())
}
