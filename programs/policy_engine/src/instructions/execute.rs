use crate::{
    get_asset_controller_account_pda, id, program::PolicyEngine, verify_cpi_program_is_token22, verify_pda, PolicyAccount, PolicyEngineAccount, PolicyEngineErrors, Side, TrackerAccount
};
use anchor_lang::{
    prelude::*,
    solana_program::{program_option::COption, sysvar::{self}},
};
use anchor_spl::token_interface::{Mint, TokenAccount};
use identity_registry::{
    program::IdentityRegistry, IdentityAccount, NO_IDENTITY_LEVEL, NO_TRACKER_LEVEL, SKIP_POLICY_LEVEL
};
use rwa_utils::META_LIST_ACCOUNT_SEED;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct ExecuteTransferHook<'info> {
    #[account()]
    pub source_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        token::token_program = anchor_spl::token_interface::spl_token_2022::id(),
    )]
    pub asset_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        token::mint = asset_mint,
        token::token_program = anchor_spl::token_interface::spl_token_2022::id(),
    )]
    // can be any token account, user must make sure it is an associated token account with relevant identity permissions
    pub destination_account: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK: can be any account
    pub owner_delegate: UncheckedAccount<'info>,
    /// CHECK: meta list account
    #[account(
        seeds = [META_LIST_ACCOUNT_SEED, asset_mint.key().as_ref()],
        bump,
    )]
    pub extra_metas_account: UncheckedAccount<'info>,
    /// CHECK: internal ix checks
    pub policy_engine_account: UncheckedAccount<'info>,
    pub identity_registry: Program<'info, IdentityRegistry>,
    /// CHECK: internal ix checks
    pub identity_registry_account: UncheckedAccount<'info>,
    /// CHECK: internal ix checks
    pub destination_identity_account: UncheckedAccount<'info>,
    /// CHECK: internal ix checks
    pub source_identity_account: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: internal ix checks
    pub destination_tracker_account: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: internal ix checks
    pub source_tracker_account: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: internal ix checks
    pub policy_account: UncheckedAccount<'info>,
    #[account(constraint = instructions_program.key() == sysvar::instructions::id())]
    /// CHECK: constraint check
    pub instructions_program: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<ExecuteTransferHook>, amount: u64) -> Result<()> {
    let asset_mint = ctx.accounts.asset_mint.key();
    if ctx.accounts.destination_account.owner == get_asset_controller_account_pda(asset_mint) {
        // if destination account is asset controller, skip enforcing token hook logic, since it is only used for revoking tokens
        return Ok(());
    }

    verify_cpi_program_is_token22(
        &ctx.accounts.instructions_program.to_account_info(),
        amount,
        asset_mint,
    )?;
    
    verify_pda(
        ctx.accounts.policy_engine_account.key(),
        &[&asset_mint.to_bytes()],
        &crate::id(),
    )?;

    verify_pda(
        ctx.accounts.policy_account.key(),
        &[&ctx.accounts.policy_engine_account.key().to_bytes()],
        &crate::id(),
    )?;

    // if policy account hasnt been created, skip enforcing token hook logic
    if ctx.accounts.policy_account.data_is_empty() {
        return Ok(());
    }

    let policy_engine_account = PolicyEngineAccount::deserialize(
        &mut &ctx.accounts.policy_engine_account.data.borrow()[8..],
    )?;

    let policy_account =
        PolicyAccount::deserialize(&mut &ctx.accounts.policy_account.data.borrow()[8..])?;

    // go through with transfer if there aren't any policies attached
    if policy_account.policies.is_empty() {
        return Ok(());
    }

    // user must have identity account setup if there are policies attached
    verify_pda(
        ctx.accounts.identity_registry_account.key(),
        &[&asset_mint.to_bytes()],
        &identity_registry::id(),
    )?;

    let destination_levels = if ctx.accounts.destination_identity_account.owner.key() == identity_registry::id() 
    && ctx.accounts.destination_identity_account.data.borrow()[..8] == *IdentityAccount::DISCRIMINATOR {
        let destination_identity_account = IdentityAccount::deserialize(
            &mut &ctx.accounts.destination_identity_account.data.borrow()[8..],
        )?;
        require!(
            ctx.accounts.destination_account.owner == destination_identity_account.owner || 
            ctx.accounts.destination_account.close_authority == COption::Some(ctx.accounts.destination_identity_account.key()), 
            PolicyEngineErrors::InvalidIdentityAccount
        );
        require!(
            destination_identity_account.identity_registry == ctx.accounts.identity_registry_account.key(),
            PolicyEngineErrors::InvalidIdentityAccount
        );
        destination_identity_account.levels
    } else {
        vec![NO_IDENTITY_LEVEL]
    };

    let source_levels = if ctx.accounts.source_identity_account.owner.key() == identity_registry::id() 
    && ctx.accounts.source_identity_account.data.borrow()[..8] == *IdentityAccount::DISCRIMINATOR {
        let source_identity_account = IdentityAccount::deserialize(
            &mut &ctx.accounts.source_identity_account.data.borrow()[8..],
        )?;
        require!(
            ctx.accounts.source_account.owner == source_identity_account.owner || 
            ctx.accounts.source_account.close_authority == COption::Some(ctx.accounts.source_identity_account.key()), 
            PolicyEngineErrors::InvalidIdentityAccount
        );
        require!(
            source_identity_account.identity_registry == ctx.accounts.identity_registry_account.key(),
            PolicyEngineErrors::InvalidIdentityAccount
        );
        source_identity_account.levels
    } else {
        vec![NO_IDENTITY_LEVEL]
    };

    // TODO: refactor skip policy level check
    // if user has identity skip level, skip enforcing policy
    if destination_levels.contains(&SKIP_POLICY_LEVEL) {
        return Ok(());
    }

    let self_transfer = ctx.accounts.source_account.close_authority != COption::None 
        && ctx.accounts.source_account.close_authority == ctx.accounts.destination_account.close_authority; 

    let source_tracker_account: Option<TrackerAccount> = if source_levels.contains(&NO_TRACKER_LEVEL) {
        None
    } else {
        require!(
            ctx.accounts.source_tracker_account.owner.key() == crate::id() 
            && ctx.accounts.source_tracker_account.data_len() == TrackerAccount::INIT_SPACE + 8,
            PolicyEngineErrors::TrackerAccountOwnerMismatch
        );
        let mut source_tracker_account = TrackerAccount::deserialize(
            &mut &ctx.accounts.source_tracker_account.data.borrow()[8..],
        )?;
        require!(
            source_tracker_account.identity_account == ctx.accounts.source_identity_account.key(),
            PolicyEngineErrors::TrackerAccountOwnerMismatch
        );

        if !self_transfer {
            source_tracker_account.update_transfer_history(
                amount,
                Clock::get()?.unix_timestamp,
                policy_engine_account.max_timeframe,
                Side::Sell
            )?;
            let source_tracker_account_data = source_tracker_account.try_to_vec()?;
            let source_tracker_account_data_len = source_tracker_account_data.len();
            ctx.accounts.source_tracker_account.data.borrow_mut()[8..8 + source_tracker_account_data_len]
                .copy_from_slice(&source_tracker_account_data);
        }
        Some(source_tracker_account)
    };
    
    let destination_tracker_account: Option<TrackerAccount> = if destination_levels.contains(&NO_TRACKER_LEVEL) {
        None
    } else {
        require!(
            ctx.accounts.destination_tracker_account.owner.key() == crate::id() 
            && ctx.accounts.destination_tracker_account.data_len() == TrackerAccount::INIT_SPACE + 8,
            PolicyEngineErrors::TrackerAccountOwnerMismatch
        );
        let mut destination_tracker_account = TrackerAccount::deserialize(
            &mut &ctx.accounts.destination_tracker_account.data.borrow()[8..],
        )?;
        require!(
            destination_tracker_account.identity_account == ctx.accounts.destination_identity_account.key(),
            PolicyEngineErrors::TrackerAccountOwnerMismatch
        );
        
        if !self_transfer {
            destination_tracker_account.update_transfer_history(
                amount,
                Clock::get()?.unix_timestamp,
                policy_engine_account.max_timeframe,
                Side::Buy,
            )?;
            let destination_tracker_account_data = destination_tracker_account.try_to_vec()?;
            let destination_tracker_account_data_len = destination_tracker_account_data.len();
            ctx.accounts.destination_tracker_account.data.borrow_mut()[8..8 + destination_tracker_account_data_len]
                .copy_from_slice(&destination_tracker_account_data);
        }
        Some(destination_tracker_account)
    };

    // evaluate policies
    policy_account.enforce_policy(
        amount,
        Clock::get()?.unix_timestamp,
        &source_levels,
        &destination_levels,
        ctx.accounts.source_account.amount,
        &source_tracker_account,
        &destination_tracker_account,
        self_transfer,
    )?;


        // new algo
        // check if skip
        // decode tracker accounts
        // update tracker accounts
        // enforce policy
        // encode tracker accounts
        // check by key where possible instead of pda
        // consider making tracker accounts zero_copy

    Ok(())
}
