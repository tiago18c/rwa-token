use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(removed_counters: Vec<u8>, added_counters: Vec<Counter>)]
pub struct ChangeCounters<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        constraint = policy_engine.authority == signer.key() || policy_engine.delegate == signer.key()
    )]
    pub signer: Signer<'info>,
    #[account(mut,
        realloc = policy_engine.to_account_info().data_len() + Counter::INIT_SPACE * added_counters.len() - Counter::INIT_SPACE * removed_counters.len(),
        realloc::zero = false,
        realloc::payer = payer,
    )]
    pub policy_engine: Box<Account<'info, PolicyEngineAccount>>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ChangeCounters>,
    removed_counters: Vec<u8>,
    added_counters: Vec<Counter>,
) -> Result<()> {
    ctx.accounts
        .policy_engine
        .update_counters(removed_counters, added_counters)?;
    Ok(())
}
