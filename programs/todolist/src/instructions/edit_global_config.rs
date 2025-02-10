use anchor_lang::prelude::*;
use crate::state::Mode;
use crate::state::GlobalConfig;

#[derive(Accounts)]
pub struct EditGlobalConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"global_config"],
        bump = global_config.bump,
        has_one = admin
    )]
    pub global_config: Account<'info, GlobalConfig>,
    pub system_program: Program<'info, System>,
}

pub fn handle_edit_global_config(ctx: Context<EditGlobalConfig>, mode: Mode) -> Result<()> {
    let global_config = &mut ctx.accounts.global_config;
    global_config.mode = mode;
    Ok(())
}
