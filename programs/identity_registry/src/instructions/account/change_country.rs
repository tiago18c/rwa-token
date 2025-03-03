use std::str::FromStr;

use crate::utils::POLICY_ENGINE_ID;
use crate::{cpi_enforce_policy_on_levels_change, state::*, ChangeCountryEvent};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[event_cpi]
pub struct ChangeCountry<'info> {
    pub payer: Signer<'info>,
    #[account(
        constraint = identity_registry.authority == signer.key()
    )]
    pub signer: Signer<'info>,
    #[account(has_one = asset_mint)]
    pub identity_registry: Box<Account<'info, IdentityRegistryAccount>>,
    #[account(
        mut,
        seeds = [identity_registry.key().as_ref(), identity_account.owner.as_ref()],
        bump,
    )]
    pub identity_account: Box<Account<'info, IdentityAccount>>,

    #[account(address = Pubkey::from_str(POLICY_ENGINE_ID).unwrap())]
    /// CHECK: hardcoded address check
    pub policy_engine_program: UncheckedAccount<'info>,
    /// CHECK: checked in cpi
    #[account(mut)]
    pub policy_engine: UncheckedAccount<'info>,
    /// CHECK: checked in cpi
    pub tracker_account: UncheckedAccount<'info>,
    /// CHECK: checked in cpi
    pub asset_mint: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<ChangeCountry>, new_country: u8, enforce_limits: bool) -> Result<()> {
    let signer_seeds = [
        &ctx.accounts.asset_mint.key().to_bytes()[..],
        &[ctx.accounts.identity_registry.bump][..],
    ];

    ctx.accounts.identity_account.set_country(new_country)?;
    let new_country = ctx.accounts.identity_account.country;

    cpi_enforce_policy_on_levels_change(
        ctx.accounts.identity_account.to_account_info(),
        ctx.accounts.identity_registry.to_account_info(),
        ctx.accounts.asset_mint.to_account_info(),
        ctx.accounts.tracker_account.to_account_info(),
        ctx.accounts.policy_engine.to_account_info(),
        ctx.accounts.policy_engine_program.to_account_info(),
        &ctx.accounts.identity_account.levels,
        new_country,
        enforce_limits,
        &[&signer_seeds[..]],
    )?;

    emit_cpi!(ChangeCountryEvent {
        identity: ctx.accounts.identity_account.key(),
        mint: ctx.accounts.identity_registry.asset_mint,
        country: new_country,
        sender: ctx.accounts.payer.key(),
    });
    Ok(())
}
