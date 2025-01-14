use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct ChangeIssuancePolicies<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        constraint = policy_engine.authority == signer.key() || policy_engine.delegate == signer.key()
    )]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub policy_engine: Box<Account<'info, PolicyEngineAccount>>,
}

pub fn handler(
    ctx: Context<ChangeIssuancePolicies>,
    issuance_policies: IssuancePolicies,
) -> Result<()> {
    ctx.accounts
        .policy_engine
        .change_issuance_policies(issuance_policies);
    Ok(())
}
