use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::PolicyEngineErrors;
#[derive(Accounts)]
pub struct SetCounters<'info> {
    pub payer: Signer<'info>,
    #[account(
        constraint = policy_engine.authority == signer.key() || policy_engine.delegate == signer.key()
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
    require!(changed_counters.len() == values.len(), PolicyEngineErrors::InvalidInstructionData);

    ctx.accounts
        .policy_engine
        .set_counters(changed_counters, values)?;
    Ok(())
}
