use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Amount must be greater than 0.")]
    InvalidAmount,
    #[msg("Overflow.")]
    Overflow,
    #[msg("Admin already exists.")]
    AdminAlreadyExists,
    #[msg("Invalid admin.")]
    InvalidAdmin,
    #[msg("Not admin.")]
    NotAdmin,
    #[msg("Invalid max length.")]
    InvalidMaxLength,
    #[msg("No available index.")]
    NoAvailableIndex,
}
