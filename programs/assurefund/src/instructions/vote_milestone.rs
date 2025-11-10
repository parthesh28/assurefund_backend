use crate::state::vote::*;
use anchor_lang::prelude::*;

#[derive(Debug, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct VoteMilestoneArgs {
    pub project_id: Pubkey,
    pub milestone_id: Pubkey,
    pub decision: bool,
}

pub fn vote_milestone(ctx: Context<VoteMilestone>, args: VoteMilestoneArgs) {
    let vote = &mut ctx.accounts.vote;

    vote.set_inner(Vote {
        vote_authority:vote.vote_authority.key(),
        project_id: args.project_id,
        milestone_id: args.milestone_id,
        decision: args.decision,
        bump: ctx.bumps.vote,
    });
}

#[derive(Accounts)]
pub struct VoteMilestone<'info> {
    #[account(mut)]
    pub vote_authority: Signer<'info>,

    #[account(
        init,
        space = Vote::DISCRIMINATOR.len() +  Vote::INIT_SPACE,
        seeds= [VOTE_SEED],
        payer = vote_authority,
        bump
    )]
    pub vote: Account<'info, Vote>,

    pub system_program: Program<'info, System>,
}
