use anchor_lang::prelude::*;
mod instructions;
use instructions::initialize::*;
use instructions::add::*;
use instructions::find_available_index::*;
use instructions::update::*;
use instructions::delete::*;
use instructions::get_list::*;
use instructions::global_config::*;
use instructions::edit_global_config::*;
mod state;
mod constant;
mod error;
use state::Mode;  // 添加这行导入
declare_id!("Fvf2JZcPnwwi1gtQkF93MjtXvCKsnThnRFjAW4Fb5H2L");

#[program]
pub mod todolist {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, max_length: u8) -> Result<()> {
        handle_initialize(ctx, max_length)
    }

    pub fn find_available_index(ctx: Context<FindAvailableIndex>) -> Result<u8> {
        handle_find_available_index(ctx)
    }

    pub fn add(ctx: Context<Add>, content: String, available_index: u8) -> Result<()> {
        handle_add(ctx, content, available_index)
    }

    pub fn update(ctx: Context<Update>, index: u8, content: String, is_completed: bool) -> Result<()> {
        handle_update(ctx, index, content, is_completed)
    }

    pub fn delete(ctx: Context<Delete>, index: u8) -> Result<()> {
        handle_delete(ctx, index)
    }

    pub fn get_list(ctx: Context<GetList>) -> Result<Vec<Pubkey>> {
        handle_get_list(ctx)
    }

    pub fn global_config(ctx: Context<GlobalConfigCtx>) -> Result<()> {
        handle_global_config(ctx)
    }

    pub fn edit_global_config(ctx: Context<EditGlobalConfig>, mode: Mode) -> Result<()> {
        handle_edit_global_config(ctx, mode)
    }
}

 
