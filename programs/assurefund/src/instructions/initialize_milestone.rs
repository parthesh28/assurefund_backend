use anchor_lang::prelude::*;

use crate::{errors::AssureFundError, state::milestone::*};

pub fn initialize_milestone(
    ctx: Context<InitializeMilestone>,
    milestone_stage: String,
    milestone_claim: u16,
    project_id: Pubkey,
) -> Result<()> {
    let milestone = &mut ctx.accounts.milestone;

    let milestone_stage_enum = MilestoneStage::try_from(milestone_stage)
        .map_err(|_| error!(AssureFundError::InvalidMilestoneStage))?;

    milestone.set_inner(Milestone {
        project_id,
        claimed_amount: milestone_claim,
        milestone_stage:milestone_stage_enum,
        attempt_number: 0,
        bump: ctx.bumps.milestone,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeMilestone<'info> {
    #[account(mut)]
    pub milestone_authority: Signer<'info>,

    #[account(
        init,
        space = Milestone::DISCRIMINATOR.len() +  Milestone::INIT_SPACE,
        seeds= [MILESTONE_SEED, ],
        payer = milestone_authority,
        bump
    )]
    pub milestone: Account<'info, Milestone>,

    pub system_program: Program<'info, System>,
}
