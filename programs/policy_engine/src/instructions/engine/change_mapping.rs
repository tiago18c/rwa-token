use anchor_lang::prelude::*;

use crate::{state::*, PolicyEngineErrors};

#[derive(Accounts)]
pub struct ChangeMapping<'info> {
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
    ctx: Context<ChangeMapping>,
    mapping_source: Vec<u8>,
    mapping_value: Vec<u8>,
) -> Result<()> {
    require!(mapping_source.len() == mapping_value.len(), PolicyEngineErrors::InvalidInstructionData);
    ctx.accounts
        .policy_engine
        .change_mapping(mapping_source, mapping_value);

    Ok(())
}
