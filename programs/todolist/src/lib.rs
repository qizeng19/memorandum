use anchor_lang::prelude::*;
mod instructions;
use instructions::initialize::*;

mod state;
 
declare_id!("8F4EfF73Yq2guFhw69v4i57CVCP9kawuWaGWkLGyMfNa");

#[program]
pub mod todolist {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        handle_initialize(ctx)
    }
}

 
