use anchor_lang::prelude::*;

pub const FUND_SEED: &[u8] = b"ASSUREFUND_PROJECT_FUND";

#[account]
#[derive(InitSpace)]
pub struct Fund{
    pub fund_authority: Pubkey,
    pub project_id: Pubkey,
    pub amount: u16,
    pub bump: u8
}