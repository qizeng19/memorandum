use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserState {
    pub user: Pubkey,
    pub index: u64,
    pub max_lenth: u64,
}
