use crate::{
    errors::AssureFundError,
    state::{PROJECT_SEED, Project, ProjectState, fund::*},
};
use anchor_lang::prelude::*;

#[derive(Clone, AnchorDeserialize, AnchorSerialize, Debug)]
pub struct FundProjectArgs {
    fund_amount: u64,
}

pub fn fund_project(ctx: Context<FundProject>, args: FundProjectArgs) -> Result<()> {
    let fund = &mut ctx.accounts.fund;
    let project = &mut ctx.accounts.project;

    require!(args.fund_amount > 0, AssureFundError::ZeroFund);

    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.fund_authority.to_account_info(),
            to: ctx.accounts.project_vault.to_account_info(),
        },
    );

    anchor_lang::system_program::transfer(cpi_ctx, args.fund_amount)?;

    if fund.amount == 0 {
        fund.set_inner(Fund {
            fund_authority: ctx.accounts.fund_authority.key(),
            project_id: project.key(),
            amount: args.fund_amount,
            bump: ctx.bumps.fund,
        });
    } else {
        fund.amount = fund
            .amount
            .checked_add(args.fund_amount)
            .ok_or(AssureFundError::Overflow)?;
    }

    project.collected_amount = project
        .collected_amount
        .checked_add(args.fund_amount)
        .ok_or(AssureFundError::Overflow)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(args: FundProjectArgs)]
pub struct FundProject<'info> {
    #[account(mut)]
    pub fund_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [PROJECT_SEED, project.project_id.as_bytes(), project.project_authority.key().as_ref()],
        bump = project.bump,
        constraint = project.project_state == ProjectState::Funding @ AssureFundError::ProjectNotFunding,

    )]
    pub project: Account<'info, Project>,

    #[account(
        init_if_needed,
        space = Fund::INIT_SPACE,
        seeds= [FUND_SEED, project.key().as_ref(), fund_authority.key().as_ref()],
        payer = fund_authority,
        bump
    )]
    pub fund: Account<'info, Fund>,

    /// CHECK: Vault PDA, created lazily on first transfer
    #[account(
        mut,
        seeds = [PROJECT_VAULT_SEED, project.key().as_ref()],
        bump
    )]
    pub project_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}
