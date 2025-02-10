use anchor_lang::prelude::*;
use crate::state::GlobalConfig;
use crate::state::Mode;
#[derive(Accounts)]
pub struct GlobalConfigCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<GlobalConfig>(),
        seeds = [b"global_config"],
        bump
    )]
    pub global_config: Account<'info, GlobalConfig>,
    pub system_program: Program<'info, System>,
}

pub fn handle_global_config(ctx: Context<GlobalConfigCtx>) -> Result<()> {
    let global_config = &mut ctx.accounts.global_config;
    global_config.admin = ctx.accounts.payer.key();
    global_config.bump = ctx.bumps.global_config;
    global_config.mode = Mode::Normal;
    Ok(())
}
