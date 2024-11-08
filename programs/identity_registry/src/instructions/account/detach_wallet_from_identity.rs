use crate::{state::*, IdentityAccount, IdentityRegistryAccount, IdentityRegistryErrors};
use anchor_lang::{prelude::*, solana_program::program_option::COption};
use anchor_spl::{
    token_2022::{freeze_account, set_authority, spl_token_2022, FreezeAccount, SetAuthority},
    token_interface::{Mint, Token2022, TokenAccount},
};
use rwa_utils::get_bump_in_seed_form;

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
