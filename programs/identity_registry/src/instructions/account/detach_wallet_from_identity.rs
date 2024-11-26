use crate::{state::*, IdentityAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
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
    
    #[account(constraint = authority.key() == identity_account.owner
            || authority.key() == identity_registry.authority,
    )]
    pub identity_account: Box<Account<'info, IdentityAccount>>,

    #[account(has_one = authority)]
    pub identity_registry: Box<Account<'info, IdentityRegistryAccount>>,
}


pub fn handler(ctx: Context<DetachWalletFromIdentity>) -> Result<()> {
    ctx.accounts.identity_account.remove_wallet()?;
    Ok(())
}
