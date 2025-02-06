use anchor_lang::prelude::*;
use crate::constant::MAX_LENGTH;

#[account]
#[derive(InitSpace)]
pub struct UserState {
    pub user: Pubkey,
    #[max_len(MAX_LENGTH as usize)]
    pub index_array: Vec<u8>,
}
