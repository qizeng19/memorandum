use anchor_lang::prelude::*;
mod instructions;
use instructions::initialize::*;
use instructions::add::*;
use instructions::find_available_index::*;
use instructions::update::*;
use instructions::delete::*;
mod state;
mod constant;
mod error;
declare_id!("8F4EfF73Yq2guFhw69v4i57CVCP9kawuWaGWkLGyMfNa");

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
}

 
