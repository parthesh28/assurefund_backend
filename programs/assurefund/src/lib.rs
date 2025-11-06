use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state; 

declare_id!("FZhS1MGJNsJknGVQ8KJGqQCMjcYdkeeAoEqUgvVd3qHV");

#[program]
pub mod assurefund {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }   
}

#[derive(Accounts)]
pub struct Initialize {}