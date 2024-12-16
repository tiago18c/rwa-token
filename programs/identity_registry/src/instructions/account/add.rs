use std::str::FromStr;

use crate::{cpi_enforce_policy_on_levels_change, state::*};
use anchor_lang::prelude::*;
use crate::utils::POLICY_ENGINE_ID;

#[derive(Accounts)]
#[instruction(level: u8)]
pub struct AddLevelToIdentityAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        constraint = identity_registry.authority == signer.key() || identity_registry.delegate == signer.key()
    )]
    pub signer: Signer<'info>,
    #[account(has_one = asset_mint)]
    pub identity_registry: Box<Account<'info, IdentityRegistryAccount>>,
    #[account(
        mut,
        seeds = [identity_registry.key().as_ref(), identity_account.owner.as_ref()],
        bump,
        realloc = identity_account.to_account_info().data_len() + IdentityLevel::INIT_SPACE, 
        realloc::zero = false,
        realloc::payer = payer,
        constraint = level != 0,
    )]
    pub identity_account: Box<Account<'info, IdentityAccount>>,
    pub system_program: Program<'info, System>,
    
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

pub fn handler(ctx: Context<AddLevelToIdentityAccount>, level: u8, expiry: i64, enforce_limits: bool) -> Result<()> {
    
    let signer_seeds = [
        &ctx.accounts.asset_mint.key().to_bytes()[..],
        &[ctx.accounts.identity_registry.bump][..],
        ];
        
    ctx.accounts.identity_account.add_level(level, expiry)?;
    let new_levels = ctx.accounts.identity_account.levels.clone();

    cpi_enforce_policy_on_levels_change(
        ctx.accounts.identity_account.to_account_info(), 
        ctx.accounts.identity_registry.to_account_info(), 
        ctx.accounts.asset_mint.to_account_info(), 
        ctx.accounts.tracker_account.to_account_info(), 
        ctx.accounts.policy_engine.to_account_info(), 
        ctx.accounts.policy_engine_program.to_account_info(), 
        &new_levels, 
        enforce_limits,
        &[&signer_seeds[..]]
    )?;
    Ok(())
}
