use crate::{state::*, CreatedIdentityEvent};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(owner: Pubkey, level: u8)]
#[event_cpi]
pub struct CreateIdentityAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        constraint = identity_registry.authority == signer.key()
    )]
    pub signer: Signer<'info>,
    #[account()]
    pub identity_registry: Box<Account<'info, IdentityRegistryAccount>>,
    #[account(
        init,
        space = 8 + IdentityAccount::INIT_SPACE,
        seeds = [identity_registry.key().as_ref(), owner.as_ref()],
        bump,
        payer = payer,
        constraint = level != 0,
    )]
    pub identity_account: Box<Account<'info, IdentityAccount>>,

    #[account(
        init,
        seeds = [owner.key().as_ref(), identity_registry.asset_mint.as_ref()],
        payer = payer,
        space = 8 + WalletIdentity::INIT_SPACE,
        bump,
    )]
    pub wallet_identity: Box<Account<'info, WalletIdentity>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateIdentityAccount>,
    owner: Pubkey,
    level: u8,
    expiry: i64,
    country: u8,
) -> Result<()> {
    ctx.accounts.identity_account.set_inner(IdentityAccount::new (
        owner,
        ctx.accounts.identity_registry.key(),
        level,
        expiry,
        country,
    ));
    ctx.accounts.wallet_identity.identity_account = ctx.accounts.identity_account.key();
    ctx.accounts.wallet_identity.wallet = owner;

    emit_cpi!(CreatedIdentityEvent {
        identity: ctx.accounts.identity_account.key(),
        mint: ctx.accounts.identity_registry.asset_mint,
        kind: level,
        sender: ctx.accounts.payer.key(),
        owner: ctx.accounts.identity_account.owner,
    });
    Ok(())
}
