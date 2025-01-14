use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(identity_filter: IdentityFilter, policy_type: PolicyType)]
pub struct AttachToPolicyEngine<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        constraint = policy_engine.authority == signer.key() || policy_engine.delegate == signer.key()
    )]
    pub signer: Signer<'info>,
    #[account(mut,
        realloc = policy_engine.to_account_info().data_len() + Policy::get_new_space(&identity_filter),
        realloc::zero = false,
        realloc::payer = payer,
    )]
    pub policy_engine: Box<Account<'info, PolicyEngineAccount>>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<AttachToPolicyEngine>,
    identity_filter: IdentityFilter,
    policy_type: PolicyType,
) -> Result<()> {
    ctx.accounts
        .policy_engine
        .update_max_timeframe(&policy_type);
    let policy_account_address = ctx.accounts.policy_engine.key();
    ctx.accounts
        .policy_engine
        .attach(policy_account_address, policy_type, identity_filter)?;
    Ok(())
}
