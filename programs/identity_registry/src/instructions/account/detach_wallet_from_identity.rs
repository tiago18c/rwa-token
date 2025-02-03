use crate::{state::*, DetachWalletFromIdentityEvent, IdentityAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[event_cpi]
pub struct DetachWalletFromIdentity<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut,
        has_one = identity_account,
        close = payer
    )]
    pub wallet_identity: Box<Account<'info, WalletIdentity>>,

    #[account(
        mut,
        // in this context, the investor is not authorized to detach the wallet
        //constraint = authority.key() == identity_account.owner || 
        constraint = authority.key() == identity_registry.authority,
        has_one = identity_registry
    )]
    pub identity_account: Box<Account<'info, IdentityAccount>>,

    pub identity_registry: Box<Account<'info, IdentityRegistryAccount>>,
}

pub fn handler(ctx: Context<DetachWalletFromIdentity>) -> Result<()> {
    ctx.accounts.identity_account.remove_wallet()?;

    emit_cpi!(DetachWalletFromIdentityEvent {
        identity: ctx.accounts.identity_account.key(),
        mint: ctx.accounts.identity_registry.asset_mint,
        wallet: ctx.accounts.wallet_identity.wallet,
        sender: ctx.accounts.payer.key(),
    });
    Ok(())
}
