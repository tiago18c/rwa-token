use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use identity_registry::{IdentityAccount, IdentityLevel, IdentityRegistryAccount};

use crate::{PolicyEngineAccount, TrackerAccount};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct NewLevelsArgs {
    pub levels: Vec<LevelExpiry>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct LevelExpiry {
    pub level: u8,
    pub expiry: i64,
}

#[derive(Accounts)]
pub struct EnforcePolicyOnLevelsChange<'info> {
    #[account(has_one = identity_registry)]
    pub identity_account: Box<Account<'info, IdentityAccount>>,
    #[account(has_one = asset_mint, signer)]
    pub identity_registry: Box<Account<'info, IdentityRegistryAccount>>,
    #[account()]
    pub asset_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        seeds = [asset_mint.key().as_ref(), identity_account.key().as_ref()],
        bump,
    )]
    pub tracker_account: Box<Account<'info, TrackerAccount>>,

    #[account(mut, has_one = asset_mint)]
    pub policy_engine: Box<Account<'info, PolicyEngineAccount>>,
}

pub fn handler(
    ctx: Context<EnforcePolicyOnLevelsChange>,
    new_levels: NewLevelsArgs,
    new_country: u8,
    enforce_limits: bool,
) -> Result<()> {
    let engine = ctx.accounts.policy_engine.as_mut();
    let balance = ctx.accounts.tracker_account.total_amount;

    let previous_levels = ctx.accounts.identity_account.levels.as_slice();
    let timestamp = Clock::get()?.unix_timestamp;

    let new_levels: Vec<IdentityLevel> = new_levels
        .levels
        .iter()
        .map(|level| IdentityLevel {
            level: level.level,
            expiry: level.expiry,
        })
        .collect();

    if balance != 0 {
        engine.update_and_enforce_policy_and_counters_on_levels_change(
            previous_levels,
            &new_levels,
            ctx.accounts.identity_account.country,
            new_country,
            timestamp,
            balance,
            enforce_limits,
        )?;
    }

    Ok(())
}
