use std::any::Any;

use anchor_lang::prelude::*;
use crate::state::*;
#[derive(Accounts)]
pub struct GetList<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(seeds = [b"user", user.key().as_ref()], bump = user_state.bump)]
    pub user_state: Account<'info, UserState>,

    pub system_program: Program<'info, System>,
}

pub fn handle_get_list(ctx: Context<GetList>) -> Result<Vec<Pubkey>> {
    let user_state = &ctx.accounts.user_state;
    let index_array = &user_state.index_array;
    let mut items_pda_list = Vec::new();
    // 遍历index_array，获取每个index对应的ListItem, 如果值为0 则跳过，否则获取ListItem
    for index in index_array {
        if *index == 0 {
            continue;
        }
        // 使用findpda 获取ListItem的pda
        let list_item_pda = Pubkey::find_program_address(
            &[b"list_item", &ctx.accounts.user.key().to_bytes(), &index.to_le_bytes()],
            ctx.program_id
        ).0;

        // 接下来需要获取ListItem的账户信息 应该怎么做
        
        items_pda_list.push(list_item_pda);
    }
    Ok((items_pda_list))
}
