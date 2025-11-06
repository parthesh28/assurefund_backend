use anchor_lang::prelude::*;

#[error_code]
pub enum AssureFundError {
    #[msg("Amount cannot be zero")]
    ZeroAmount,

     #[msg("Invalid milestone stage provided.")]
    InvalidMilestoneStage,
}