use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(hash: String)]
pub struct DetachFromPolicyEngine<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        constraint = policy_engine.authority == signer.key() || policy_engine.delegate == signer.key()
    )]
    pub signer: Signer<'info>,
    #[account(mut,
        realloc = policy_engine.to_account_info().data_len() - policy_engine.get_policy_space(&hash)?,
        realloc::zero = false,
        realloc::payer = payer,
    )]
    pub policy_engine: Box<Account<'info, PolicyEngineAccount>>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DetachFromPolicyEngine>, hash: String) -> Result<()> {
    ctx.accounts.policy_engine.detach(hash)?;
    Ok(())
}
