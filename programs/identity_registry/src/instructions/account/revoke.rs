use crate::{state::*, RemovedIdentityEvent};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(owner: Pubkey)]
#[event_cpi]
pub struct RevokeIdentityAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        constraint = identity_registry.authority == signer.key() || identity_registry.delegate == signer.key()
    )]
    pub signer: Signer<'info>,
    #[account(
        seeds = [identity_registry.asset_mint.key().as_ref()],
        bump,
    )]
    pub identity_registry: Box<Account<'info, IdentityRegistryAccount>>,
    #[account(
        mut,
        close = payer,
        seeds = [identity_registry.key().as_ref(), identity_account.owner.as_ref()],
        bump,
        constraint = identity_account.owner == owner,
        constraint = identity_account.num_wallets == 1
    )]
    pub identity_account: Box<Account<'info, IdentityAccount>>,

    #[account(
        mut,
        close = payer,
        seeds = [owner.key().as_ref(), identity_registry.asset_mint.as_ref()],
        bump,
        has_one = identity_account,
    )]
    pub wallet_identity: Box<Account<'info, WalletIdentity>>,
}

pub fn handler(ctx: Context<RevokeIdentityAccount>, _owner: Pubkey) -> Result<()> {
    emit_cpi!(RemovedIdentityEvent {
        identity: ctx.accounts.identity_account.key(),
        mint: ctx.accounts.identity_registry.asset_mint,
        sender: ctx.accounts.payer.key(),
        kind: ctx.accounts.identity_account.levels[0].level,
        owner: ctx.accounts.identity_account.owner,
    });
    Ok(())
}

