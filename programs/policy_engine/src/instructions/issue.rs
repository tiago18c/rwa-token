use crate::{
    PolicyEngineAccount, TrackerAccount
};
use anchor_lang::
    prelude::*
;
use anchor_spl::token_interface::{Mint, TokenAccount};
use identity_registry::{
    IdentityAccount, IdentityRegistryAccount, SKIP_POLICY_LEVEL
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct EnforcePolicyIssuanceAccounts<'info> {
    //#[account(signer, has_one = asset_mint)]
    pub asset_controller: Signer<'info>,
    #[account(
        token::token_program = anchor_spl::token_interface::spl_token_2022::id(),
        mint::authority = asset_controller
    )]
    pub asset_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut, has_one = asset_mint)]
    pub policy_engine: Box<Account<'info, PolicyEngineAccount>>,
    // can be any token account, user must make sure it is an associated token account with relevant identity permissions
    #[account(
        token::mint = asset_mint,
        token::token_program = anchor_spl::token_interface::spl_token_2022::id(),
    )]
    pub destination_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(has_one = asset_mint)]
    pub identity_registry: Box<Account<'info, IdentityRegistryAccount>>,
    #[account(has_one = identity_registry)]
    pub identity_account: Box<Account<'info, IdentityAccount>>,
    #[account(mut, has_one = identity_account)]
    pub destination_tracker_account: Box<Account<'info, TrackerAccount>>,
}

pub fn handler(ctx: Context<EnforcePolicyIssuanceAccounts>, amount: u64) -> Result<()> {

    // TODO: refactor skip policy level check
    // if user has identity skip level, skip enforcing policy
    if ctx.accounts.identity_account.levels.contains(&SKIP_POLICY_LEVEL) {
        return Ok(());
    }


    let tracker_account : &mut TrackerAccount = &mut ctx.accounts.destination_tracker_account;

    tracker_account.update_balance_mint(amount)?;

    if !ctx.accounts.policy_engine.enforce_policy_issuance {
        return Ok(());
    }

    if tracker_account.total_amount == amount {
        let changed_counters = ctx.accounts.policy_engine.increase_holders_count(&ctx.accounts.identity_account.levels, ctx.accounts.identity_account.country)?;
        ctx.accounts.policy_engine.enforce_counters_on_increment(&changed_counters)?;
    }

    // evaluate policies
    ctx.accounts.policy_engine.enforce_policy_issuance(
        amount,
        Clock::get()?.unix_timestamp,
        &ctx.accounts.identity_account.levels,
        ctx.accounts.identity_account.country,
        Some(&tracker_account),
    )?;
    Ok(())
}
