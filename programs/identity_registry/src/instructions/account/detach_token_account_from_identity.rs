use crate::{state::*, IdentityAccount, IdentityRegistryAccount, IdentityRegistryErrors};
use anchor_lang::{prelude::*, solana_program::program_option::COption};
use anchor_spl::{
    token_2022::{freeze_account, set_authority, spl_token_2022, FreezeAccount, SetAuthority},
    token_interface::{Mint, Token2022, TokenAccount},
};
use rwa_utils::get_bump_in_seed_form;

#[derive(Accounts)]
pub struct DetachTokenAccountFromIdentity<'info> {
    /// CHECK: checked by identity account seeds
    pub owner: UncheckedAccount<'info>,
    #[account(mut)]
    pub asset_controller: Signer<'info>,
    #[account(
        seeds = [identity_registry.key().as_ref(), owner.key().as_ref()],
        bump,
        constraint = identity_account.identity_registry == identity_registry.key()
    )]
    pub identity_account: Box<Account<'info, IdentityAccount>>,
    #[account(
        constraint = identity_registry.asset_mint == asset_mint.key()
    )]
    pub identity_registry: Box<Account<'info, IdentityRegistryAccount>>,
    #[account(
        mut,
        token::mint = identity_registry.asset_mint,
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mint::authority = asset_controller)]
    pub asset_mint: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Program<'info, Token2022>,
}

impl<'info> DetachTokenAccountFromIdentity<'info> {
    
    fn set_close_authority(&self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let accounts = SetAuthority {
            account_or_mint: self.token_account.to_account_info(),
            current_authority: self.identity_account.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, signer_seeds);
        set_authority(
            cpi_ctx,
            spl_token_2022::instruction::AuthorityType::CloseAccount,
            None,
        )?;
        Ok(())
    }
    
}

pub fn handler(ctx: Context<DetachTokenAccountFromIdentity>) -> Result<()> {
    require!(ctx.accounts.identity_registry.require_identity_creation, IdentityRegistryErrors::IdentityCreationRequired);
    let identity_registry = ctx.accounts.identity_registry.key();
    let owner = ctx.accounts.owner.key();
    let signer_seeds = [
        identity_registry.as_ref(),
        owner.as_ref(),
        &get_bump_in_seed_form(&ctx.bumps.identity_account),
    ];
    ctx.accounts.set_close_authority(&[&signer_seeds])?;
    ctx.accounts.identity_account.remove_token_account()?;
    Ok(())
}
