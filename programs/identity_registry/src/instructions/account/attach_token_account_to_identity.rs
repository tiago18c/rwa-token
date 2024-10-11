use crate::{state::*, IdentityAccount, IdentityRegistryAccount, IdentityRegistryErrors};
use anchor_lang::{prelude::*, solana_program::program_option::COption};
use anchor_spl::{
    token_2022::{freeze_account, FreezeAccount},
    token_interface::{Mint, Token2022, TokenAccount},
};

#[derive(Accounts)]
pub struct AttachTokenAccountToIdentity<'info> {
    #[account(mut)]
    pub asset_controller: Signer<'info>,
    #[account(
        constraint = identity_account.identity_registry == identity_registry.key()
    )]
    pub identity_account: Box<Account<'info, IdentityAccount>>,
    pub identity_registry: Box<Account<'info, IdentityRegistryAccount>>,
    #[account(
        mut,
        token::mint = identity_registry.asset_mint,
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mint::authority = asset_controller)]
    pub asset_mint: Box<InterfaceAccount<'info, Mint>>,
}


pub fn handler(ctx: Context<AttachTokenAccountToIdentity>) -> Result<()> {
    ctx.accounts.identity_account.add_token_account()?;
    Ok(())
}
