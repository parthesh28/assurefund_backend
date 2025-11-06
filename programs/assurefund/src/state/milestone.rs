use anchor_lang::prelude::*;

pub const MILESTONE_SEED: &[u8] = b"ASSUREFUND_PROJECT_MILESTONE";

#[account]
#[derive(InitSpace)]
pub struct Milestone{
    pub project_id: Pubkey,
    pub claimed_amount: u16,
    pub milestone_stage: MilestoneStage,
    pub attempt_number: u8,
    pub bump: u8
}

#[derive(Clone, InitSpace, AnchorDeserialize, AnchorSerialize)]
pub enum MilestoneStage{
    Design,
    Development,
    Testing, 
    Deliver,
    Upfront
}

impl TryFrom<String> for MilestoneStage {
    type Error = ();

    fn try_from(value: String) -> std::result::Result<MilestoneStage, ()> {
        match value.to_lowercase().as_str() {
            "design" => Ok(Self::Design),
            "development" => Ok(Self::Development),
            "testing" => Ok(Self::Testing),
            "deliver" => Ok(Self::Deliver),
            "upfront" => Ok(Self::Upfront),
            _ => Err(()),
        }
    }
}