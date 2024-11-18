use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(level: u8)]
pub struct RefreshLevelToIdentityAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        constraint = identity_registry.authority == signer.key() || identity_registry.delegate == signer.key()
    )]
    pub signer: Signer<'info>,
    #[account()]
    pub identity_registry: Box<Account<'info, IdentityRegistryAccount>>,
    #[account(
        mut,
        seeds = [identity_registry.key().as_ref(), identity_account.owner.as_ref()],
        bump,
        constraint = level != 0,
    )]
    pub identity_account: Box<Account<'info, IdentityAccount>>,
}

pub fn handler(ctx: Context<RefreshLevelToIdentityAccount>, level: u8, expiry: i64) -> Result<()> {
    ctx.accounts.identity_account.refresh_level(level, expiry)?;
    Ok(())
}
