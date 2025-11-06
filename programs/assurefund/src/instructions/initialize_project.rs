use anchor_lang::prelude::*;

use crate::{errors::AssureFundError, state::project::*};

pub fn initialize_project(
    ctx: Context<InitializeProject>,
    project_id: String,
    project_amount: u16,
) -> Result<()> {
    let project = &mut ctx.accounts.project;

    require!(project_amount > 0, AssureFundError::ZeroAmount);

    project.set_inner(Project {
        project_authority: ctx.accounts.project_authority.key(),
        project_id: project_id,
        amount_requred: project_amount,
        amount_collect: 0,
        project_state: ProjectState::Funding,
        bump: ctx.bumps.project,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(project_id: String)]
pub struct InitializeProject<'info> {
    #[account(mut)]
    pub project_authority: Signer<'info>,

    #[account(
        init,
        space = Project::DISCRIMINATOR.len() +  Project::INIT_SPACE,
        seeds= [PROJECT_SEED, project_id.as_bytes(), project_authority.key().as_ref()],
        payer = project_authority,
        bump
    )]
    pub project: Account<'info, Project>,

    pub system_program: Program<'info, System>,
}
