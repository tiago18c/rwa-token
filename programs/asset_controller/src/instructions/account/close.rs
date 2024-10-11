use anchor_lang::{prelude::*, solana_program::program_option::COption};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{close_account, CloseAccount, Mint, Token2022, TokenAccount},
};
use identity_registry::{program::IdentityRegistry, IdentityAccount, IdentityRegistryAccount};
use rwa_utils::get_bump_in_seed_form;

use crate::AssetControllerAccount;

#[derive(Accounts)]
#[instruction()]
pub struct CloseTokenAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account()]
    pub owner: Signer<'info>,
    /// CHECK: checked by identity account seeds
    pub identity_account_owner: UncheckedAccount<'info>,
    #[account()]
    pub asset_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        seeds = [asset_mint.key().as_ref()],
        bump,
    )]
    pub asset_controller: Box<Account<'info, AssetControllerAccount>>,
    #[account(signer,
        constraint = identity_account.identity_registry == identity_registry.key(),
        constraint = COption::Some(identity_account.key()) == token_account.close_authority
    )]
    pub identity_account: Box<Account<'info, IdentityAccount>>,
    #[account(has_one = asset_mint)]
    pub identity_registry: Box<Account<'info, IdentityRegistryAccount>>,
    #[account(
        mut,
        associated_token::mint = asset_mint,
        associated_token::authority = owner
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub identity_registry_program: Program<'info, IdentityRegistry>,
}

impl<'info> CloseTokenAccount<'info> {
    fn detach_token_account_from_identity(&self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let accounts = identity_registry::cpi::accounts::DetachTokenAccountFromIdentity {
            identity_account: self.identity_account.to_account_info(),
            identity_registry: self.identity_registry.to_account_info(),
            token_account: self.token_account.to_account_info(),
            asset_mint: self.asset_mint.to_account_info(),
            asset_controller: self.asset_controller.to_account_info(),
            owner: self.identity_account_owner.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.identity_registry_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        identity_registry::cpi::detach_token_account_from_identity(cpi_ctx)?;
        Ok(())
    }
}

pub fn handler(ctx: Context<CloseTokenAccount>) -> Result<()> {
    let asset_mint = ctx.accounts.asset_mint.key();
    let signer_seeds = [
        asset_mint.as_ref(),
        &get_bump_in_seed_form(&ctx.bumps.asset_controller),
    ];

    ctx.accounts.detach_token_account_from_identity(&[&signer_seeds])?;
    
    let accounts = CloseAccount {
        authority: ctx.accounts.owner.to_account_info(),
        account: ctx.accounts.token_account.to_account_info(),
        destination: ctx.accounts.payer.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts);
    close_account(cpi_ctx)?;
    Ok(())
}
