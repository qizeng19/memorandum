use anchor_lang::prelude::*;
use crate::constant::{MAX_LENGTH, MAX_ITEM_CONTENT_LENGTH};

#[account]
#[derive(InitSpace)]
pub struct UserState {
    pub user: Pubkey,
    #[max_len(MAX_LENGTH as usize)]
    pub index_array: Vec<u8>,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct ListItem {
    pub user: Pubkey,
    pub index: u8,
    #[max_len(MAX_ITEM_CONTENT_LENGTH as usize)]
    pub content: String,
    pub is_completed: bool,
    pub bump: u8,
}

#[account]
pub struct GlobalConfig {
    pub admin: Pubkey,
    pub mode: Mode,
    pub bump: u8,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Paused,
    Terminated,
}