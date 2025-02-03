use anchor_lang::prelude::*;

use crate::{state::*, DetachPolicyEvent};

#[derive(Accounts)]
#[instruction(hash: String)]
#[event_cpi]
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
    let policy = ctx.accounts.policy_engine.detach(hash)?;

    emit_cpi!(DetachPolicyEvent {
        mint: ctx.accounts.policy_engine.asset_mint,
        policy_type: policy.policy_type,
        identity_filter: policy.identity_filter,
        custom_error: policy.custom_error
    });
    Ok(())
}
