use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{mint_to, Mint, MintTo, Token2022, TokenAccount},
};
use identity_registry::{IdentityAccount, IdentityRegistryAccount, WalletIdentity};
use policy_engine::{program::PolicyEngine, PolicyEngineAccount, TrackerAccount};
use rwa_utils::get_bump_in_seed_form;

use crate::{AssetControllerAccount, AssetControllerErrors};

#[derive(Accounts)]
#[instruction()]
pub struct IssueTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub asset_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        seeds = [asset_mint.key().as_ref()],
        bump,
        constraint = asset_controller.authority == authority.key(),
    )]
    pub asset_controller: Box<Account<'info, AssetControllerAccount>>,
    /// CHECK: can be any account
    pub to: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::token_program = token_program,
        associated_token::mint = asset_mint,
        associated_token::authority = to,
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(has_one = asset_mint)]
    pub identity_registry: Box<Account<'info, IdentityRegistryAccount>>,
    #[account(has_one = identity_registry)]
    pub identity_account: Box<Account<'info, IdentityAccount>>,
    #[account(mut, has_one = asset_mint)]
    pub tracker_account: Box<Account<'info, TrackerAccount>>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub policy_engine_program: Program<'info, PolicyEngine>,
    #[account(mut)]
    pub policy_engine: Box<Account<'info, PolicyEngineAccount>>,
    /// CHECK: checked in handler
    pub wallet_identity_account: Option<UncheckedAccount<'info>>,
}

impl<'info> IssueTokens<'info> {
    fn issue_tokens(&self, amount: u64, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let accounts = MintTo {
            mint: self.asset_mint.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.asset_controller.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    fn enforce_policy_issuance(&self, amount: u64, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let accounts = policy_engine::cpi::accounts::EnforcePolicyIssuanceAccounts {
            asset_mint: self.asset_mint.to_account_info(),
            policy_engine: self.policy_engine.to_account_info(),
            destination_account: self.token_account.to_account_info(),
            identity_registry: self.identity_registry.to_account_info(),
            identity_account: self.identity_account.to_account_info(),
            destination_tracker_account: self.tracker_account.to_account_info(),
            asset_controller: self.asset_controller.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.policy_engine_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        policy_engine::cpi::enforce_policy_issuance(cpi_ctx, amount)?;
        Ok(())
    }
}

pub fn handler(ctx: Context<IssueTokens>, amount: u64) -> Result<()> {
    // either to is the wallet related to the identity account
    // or we have a wallet identity account that links a wallet to an identity account
    let wallet_identity = ctx.accounts.wallet_identity_account.as_ref().and_then(|v| WalletIdentity::try_deserialize(&mut &v.data.borrow()[..]).ok());
    
    require!(
        ctx.accounts.to.key() == ctx.accounts.identity_account.owner 
        || (wallet_identity.is_some() 
            && wallet_identity.as_ref().unwrap().wallet == ctx.accounts.to.key() 
            && wallet_identity.as_ref().unwrap().identity_account == ctx.accounts.identity_account.key()),
        AssetControllerErrors::InvalidIdentityAccounts
    );


    let asset_mint = ctx.accounts.asset_mint.key();
    let signer_seeds = [
        asset_mint.as_ref(),
        &get_bump_in_seed_form(&ctx.bumps.asset_controller),
    ];
    ctx.accounts.issue_tokens(amount, &[&signer_seeds])?;
    ctx.accounts.enforce_policy_issuance(amount, &[&signer_seeds])?;
    Ok(())
}
