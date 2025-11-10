use anchor_lang::prelude::*;

pub const PROJECT_SEED: &[u8] = b"ASSUREFUND_PROJECT";

#[account]
#[derive(InitSpace)]
pub struct Project{
    pub project_authority : Pubkey,
    #[max_len(32)]
    pub project_id: String,
    pub target_amount: u16,
    pub collected_amount: u16,
    pub project_state: ProjectState,
    pub bump: u8
}

#[derive(Clone, InitSpace, AnchorSerialize, AnchorDeserialize)]
pub enum ProjectState{
    Funding,
    Development,
    Failed, 
    Completed
}

