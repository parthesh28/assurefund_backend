use anchor_lang::{prelude::*};

pub const VOTE_SEED: &[u8] = b"ASSUREFUND_MILESTONE_VOTE";

#[account]
#[derive(InitSpace)]
pub struct Vote{
    pub vote_authority: Pubkey,
    pub project_id: Pubkey,
    pub milestone_id: Pubkey,
    pub decision: bool,
    pub bump: u8
}