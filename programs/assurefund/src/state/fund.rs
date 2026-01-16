use anchor_lang::prelude::*;

pub const FUND_SEED: &[u8] = b"ASSUREFUND_PROJECT_FUND";
pub const PROJECT_VAULT_SEED: &[u8] = b"ASSUREFUND_PROJECT_VAULT";


#[account]
#[derive(InitSpace)]
pub struct Fund{
    pub fund_authority: Pubkey,
    pub project_id: Pubkey,
    pub amount: u64,
    pub bump: u8
}