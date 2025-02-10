use crate::state::*;
use anchor_lang::prelude::*;
use crate::error::ErrorCode;
#[derive(Accounts)]
#[instruction(index: u8)]
pub struct Update<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"list_item", user.key().as_ref(), &index.to_le_bytes()],
        bump = list_item.bump
    )]
    pub list_item: Account<'info, ListItem>,

    #[account(
        seeds = [b"global_config"],
        bump = global_config.bump
    )]
    pub global_config: Account<'info, GlobalConfig>,

    pub system_program: Program<'info, System>,
}

pub fn handle_update(ctx: Context<Update>, _index: u8, content: String, is_completed: bool) -> Result<()> {
    let global_config = &ctx.accounts.global_config;
    if global_config.mode == Mode::Paused {
        return Err(ErrorCode::GlobalConfigPaused.into());
    }
    let list_item = &mut ctx.accounts.list_item;
    list_item.content = content;
    list_item.is_completed = is_completed;
    Ok(())
}
