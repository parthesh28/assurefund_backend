use anchor_lang::prelude::*;

use crate::{errors::AssureFundError, state::milestone::*};

#[derive(Clone, Debug, AnchorDeserialize, AnchorSerialize)]
pub struct InitializeMilestoneArgs{
   pub milestone_type: MilestoneType, 
   pub milestone_claim: u16,
   pub project_id: Pubkey,
}

pub fn initialize_milestone(
    ctx: Context<InitializeMilestone>,
    args: InitializeMilestoneArgs
) -> Result<()> {
    let milestone = &mut ctx.accounts.milestone;
    
    let milestone_type = MilestoneType::try_from(args.milestone_type)
        .map_err(|_| error!(AssureFundError::InvalidMilestoneType))?;

    milestone.set_inner(Milestone {
        project_id: args.project_id,
        milestone_claim: args.milestone_claim,
        attempt_number: 0,
        milestone_status: MilestoneState::Created,
        milestone_type: milestone_type,
        vote_against:0,
        vote_for: 0,
        bump: ctx.bumps.milestone,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(args: InitializeMilestoneArgs)]
pub struct InitializeMilestone<'info> {
    #[account(mut)]
    pub milestone_authority: Signer<'info>,

    #[account(
        init,
        space = Milestone::DISCRIMINATOR.len() +  Milestone::INIT_SPACE,
        seeds= [MILESTONE_SEED, args.project_id.key().as_ref(), &[args.milestone_type as u8], milestone_authority.key().as_ref()],
        payer = milestone_authority,
        bump
    )]
    pub milestone: Box<Account<'info, Milestone>>,

    pub system_program: Program<'info, System>,
}