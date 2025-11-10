use crate::{ errors::AssureFundError, state::fund::*};
use anchor_lang::prelude::*;

#[derive(Clone, AnchorDeserialize, AnchorSerialize, Debug)]
pub struct FundProjectArgs{
    project_id: Pubkey,
    fund_amount: u16,
    project_authority: Pubkey   
}

pub fn fund_project(
    ctx: Context<FundProject>,
    args: FundProjectArgs
) -> Result<()> {
    let fund = &mut ctx.accounts.fund;

    require!(args.fund_amount > 0, AssureFundError::ZeroFund);

    fund.set_inner(Fund {
        fund_authority: ctx.accounts.fund_authority.key(),
        project_id: args.project_id,
        amount: args.fund_amount,
        bump: ctx.bumps.fund,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(args: FundProjectArgs)]
pub struct FundProject<'info> {
    #[account(mut)]
    pub fund_authority: Signer<'info>,

    #[account(
        init,
        space = Fund::DISCRIMINATOR.len() +  Fund::INIT_SPACE,
        seeds= [FUND_SEED, args.project_id.key().as_ref(), fund_authority.key().as_ref()],
        payer = fund_authority,
        bump
    )]
    pub fund: Account<'info, Fund>,

    pub system_program: Program<'info, System>,
}