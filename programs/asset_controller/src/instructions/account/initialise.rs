use crate::{state::*, AssetControllerErrors};
use anchor_lang::{prelude::*, solana_program::program_option::COption};
use anchor_spl::{
    token_2022::{set_authority, thaw_account, SetAuthority, ThawAccount},
    token_interface::{Mint, Token2022, TokenAccount},
};
use identity_registry::{program::IdentityRegistry, IdentityAccount, IdentityRegistryAccount};
use rwa_utils::get_bump_in_seed_form;

#[derive(Accounts)]
#[instruction()]
pub struct InitializeTokenAccount<'info> {
    pub identity_account_owner: Signer<'info>,
    pub owner: Signer<'info>,
    #[account(signer,
        constraint = identity_account.identity_registry == identity_registry.key(),
        constraint = identity_account.owner == identity_account_owner.key()
    )]
    pub identity_account: Box<Account<'info, IdentityAccount>>,
    #[account(has_one = asset_mint)]
    pub identity_registry: Box<Account<'info, IdentityRegistryAccount>>,
    #[account(mut)]
    pub asset_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        seeds = [asset_mint.key().as_ref()],
        bump,
    )]
    pub asset_controller: Box<Account<'info, AssetControllerAccount>>,
    #[account(mut, token::authority = owner, token::mint = asset_mint)]
    pub token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Program<'info, Token2022>,
    pub identity_registry_program: Program<'info, IdentityRegistry>,
}

impl<'info> InitializeTokenAccount<'info> {
    fn thaw_tokens(&self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let accounts = ThawAccount {
            mint: self.asset_mint.to_account_info(),
            authority: self.asset_controller.to_account_info(),
            account: self.token_account.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        thaw_account(cpi_ctx)?;
        Ok(())
    }

    fn set_close_authority(&self) -> Result<()> {
        let accounts = SetAuthority {
            account_or_mint: self.token_account.to_account_info(),
            current_authority: self.owner.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), accounts);
        set_authority(
            cpi_ctx,
            spl_token_2022::instruction::AuthorityType::CloseAccount,
            Some(self.identity_account.key()),
        )?;
        Ok(())
    }

    fn attach_token_account_to_identity(&self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let accounts = identity_registry::cpi::accounts::AttachTokenAccountToIdentity {
            identity_account: self.identity_account.to_account_info(),
            identity_registry: self.identity_registry.to_account_info(),
            token_account: self.token_account.to_account_info(),
            asset_mint: self.asset_mint.to_account_info(),
            asset_controller: self.asset_controller.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.identity_registry_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        identity_registry::cpi::attach_token_account_to_identity(cpi_ctx)?;
        Ok(())
    }
}

pub fn handler(ctx: Context<InitializeTokenAccount>) -> Result<()> {
    require!(
        ctx.accounts.token_account.close_authority == COption::None,
        AssetControllerErrors::TokenAccountAlreadyInitialized
    );
    let asset_mint = ctx.accounts.asset_mint.key();
    let signer_seeds = [
        asset_mint.as_ref(),
        &get_bump_in_seed_form(&ctx.bumps.asset_controller),
    ];
    ctx.accounts.thaw_tokens(&[&signer_seeds])?;
    ctx.accounts.set_close_authority()?;
    ctx.accounts.attach_token_account_to_identity(&[&signer_seeds])?;
    Ok(())
}
