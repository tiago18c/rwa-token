use crate::{state::*, IdentityAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DetachWalletFromIdentity<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(mut,
        has_one = identity_account,
        close = owner
    )]
    pub wallet_identity: Box<Account<'info, WalletIdentity>>,
    
    #[account(
        has_one = owner
    )]
    pub identity_account: Box<Account<'info, IdentityAccount>>,
}


pub fn handler(ctx: Context<DetachWalletFromIdentity>) -> Result<()> {
    ctx.accounts.identity_account.remove_wallet()?;
    Ok(())
}
