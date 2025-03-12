use anchor_lang::prelude::*;

use crate::error::PolicyEngineErrors;
use crate::{state::*, SetCounterValueEvent};

#[derive(Accounts)]
#[event_cpi]
pub struct SetCounters<'info> {
    pub payer: Signer<'info>,
    #[account(
        constraint = policy_engine.authority == signer.key()
    )]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub policy_engine: Box<Account<'info, PolicyEngineAccount>>,
}

pub fn handler(
    ctx: Context<SetCounters>,
    changed_counters: Vec<u8>,
    values: Vec<u64>,
) -> Result<()> {
    require!(
        changed_counters.len() == values.len(),
        PolicyEngineErrors::InvalidInstructionData
    );

    ctx.accounts
        .policy_engine
        .set_counters(changed_counters.clone(), values.clone())?;

    emit_cpi!(SetCounterValueEvent {
        mint: ctx.accounts.policy_engine.asset_mint,
        counters: changed_counters,
        values
    });
    Ok(())
}
