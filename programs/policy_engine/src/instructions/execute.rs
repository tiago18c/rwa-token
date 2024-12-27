use crate::{
    get_asset_controller_account_pda, assert_is_transferring, verify_pda, PolicyEngineAccount, PolicyEngineErrors, Side, TrackerAccount
};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount};
use identity_registry::{
    program::IdentityRegistry, IdentityAccount, NO_IDENTITY_LEVEL, NO_TRACKER_LEVEL, SKIP_POLICY_LEVEL, WalletIdentity
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
    #[account(mut)]
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

    /// CHECK: internal ix checks
    pub destination_wallet_identity: UncheckedAccount<'info>,
    /// CHECK: internal ix checks
    pub source_wallet_identity: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<ExecuteTransferHook>, amount: u64) -> Result<()> {
    let asset_mint = ctx.accounts.asset_mint.key();
    if ctx.accounts.destination_account.owner == get_asset_controller_account_pda(asset_mint) {
        // if destination account is asset controller, skip enforcing token hook logic, since it is only used for revoking tokens
        return Ok(());
    }

    assert_is_transferring(&ctx.accounts.source_account.to_account_info())?;
    assert_is_transferring(&ctx.accounts.destination_account.to_account_info())?;

    require!(
        ctx.accounts.policy_engine_account.owner == &crate::id() 
        && ctx.accounts.policy_engine_account.data.borrow()[..8] == *PolicyEngineAccount::DISCRIMINATOR, 
        PolicyEngineErrors::InvalidPolicyEngineAccount
    );

    let mut policy_engine_account = Box::new(PolicyEngineAccount::deserialize(
        &mut &ctx.accounts.policy_engine_account.data.borrow()[8..],
    )?);

    require!(policy_engine_account.asset_mint == asset_mint, PolicyEngineErrors::InvalidPolicyEngineAccount);

    // if policy account hasnt been created, skip enforcing token hook logic
    if policy_engine_account.policies.is_empty() && policy_engine_account.counters.is_empty() {
        return Ok(());
    }

    // user must have identity account setup if there are policies attached
    verify_pda(
        ctx.accounts.identity_registry_account.key(),
        &[&asset_mint.to_bytes()],
        &identity_registry::id(),
    )?;

    let (destination_levels, destination_country) = if ctx.accounts.destination_identity_account.owner.key() == identity_registry::id() 
    && ctx.accounts.destination_identity_account.data.borrow()[..8] == *IdentityAccount::DISCRIMINATOR {
        let destination_identity_account = Box::new(IdentityAccount::deserialize(
            &mut &ctx.accounts.destination_identity_account.data.borrow()[8..],
        )?);

        if destination_identity_account.owner != ctx.accounts.destination_account.owner {
            //deserialize wallet identity
            let destination_wallet_identity = WalletIdentity::deserialize(
                &mut &ctx.accounts.destination_wallet_identity.data.borrow()[8..],
            )?;

            require!(
                ctx.accounts.destination_account.owner == destination_wallet_identity.wallet,
                PolicyEngineErrors::InvalidIdentityAccount
            );
        } 

        require!(
            destination_identity_account.identity_registry == ctx.accounts.identity_registry_account.key(),
            PolicyEngineErrors::InvalidIdentityAccount
        );
        (destination_identity_account.levels, destination_identity_account.country)
    } else {
        (vec![NO_IDENTITY_LEVEL], 0)
    };

    let (source_levels, source_country) = if ctx.accounts.source_identity_account.owner.key() == identity_registry::id() 
    && ctx.accounts.source_identity_account.data.borrow()[..8] == *IdentityAccount::DISCRIMINATOR {
        let source_identity_account = Box::new(IdentityAccount::deserialize(
            &mut &ctx.accounts.source_identity_account.data.borrow()[8..],
        )?);
        
        if source_identity_account.owner != ctx.accounts.source_account.owner {
            //deserialize wallet identity
            let source_wallet_identity = WalletIdentity::deserialize(
                &mut &ctx.accounts.source_wallet_identity.data.borrow()[8..],
            )?;

            require!(
                ctx.accounts.source_account.owner == source_wallet_identity.wallet,
                PolicyEngineErrors::InvalidIdentityAccount
            );
        } 
        require!(
            source_identity_account.identity_registry == ctx.accounts.identity_registry_account.key(),
            PolicyEngineErrors::InvalidIdentityAccount
        );
        (source_identity_account.levels, source_identity_account.country)
    } else {
        (vec![NO_IDENTITY_LEVEL], 0)
    };

    // TODO: refactor skip policy level check
    // if user has identity skip level, skip enforcing policy
    if destination_levels.contains(&SKIP_POLICY_LEVEL) {
        return Ok(());
    }

    let self_transfer = ctx.accounts.source_account.owner == ctx.accounts.destination_account.owner
    || ctx.accounts.source_identity_account.key() == ctx.accounts.destination_identity_account.key(); 

    let source_tracker_account: Option<Box<TrackerAccount>> = if source_levels.contains(&NO_TRACKER_LEVEL) {
        None
    } else {
        require!(
            ctx.accounts.source_tracker_account.owner.key() == crate::id() 
            && ctx.accounts.source_tracker_account.data_len() == TrackerAccount::INIT_SPACE + 8,
            PolicyEngineErrors::TrackerAccountOwnerMismatch
        );
        let mut source_tracker_account = Box::new(TrackerAccount::deserialize(
            &mut &ctx.accounts.source_tracker_account.data.borrow()[8..],
        )?);
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

    let destination_tracker_account: Option<Box<TrackerAccount>> = if destination_levels.contains(&NO_TRACKER_LEVEL) {
        None
    } else {
        require!(
            ctx.accounts.destination_tracker_account.owner.key() == crate::id() 
            && ctx.accounts.destination_tracker_account.data_len() == TrackerAccount::INIT_SPACE + 8,
            PolicyEngineErrors::TrackerAccountOwnerMismatch
        );
        let mut destination_tracker_account = Box::new(TrackerAccount::deserialize(
            &mut &ctx.accounts.destination_tracker_account.data.borrow()[8..],
        )?);
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

    let source_balance = if let Some(sta) = &source_tracker_account { sta.total_amount } else { ctx.accounts.source_account.amount };
    let destination_balance = if let Some(dta) = &destination_tracker_account { dta.total_amount } else { ctx.accounts.destination_account.amount };

    let timestamp = Clock::get()?.unix_timestamp;

    msg!("source_levels: {:?}", source_levels);
    msg!("destination_levels: {:?}", destination_levels);

    msg!("source balance: {:?}", source_balance);
    msg!("destination balance: {:?}", destination_balance);

    msg!("amount: {:?}", amount);

    if !self_transfer {
        let decreased_counters = if source_balance == 0 {
            // source has 0 balance
            policy_engine_account.decrease_holders_count(&source_levels, source_country)?
        } else {
            vec![]
        };
        let increased_counters = if destination_balance == amount {
            // destination has 0 balance
            policy_engine_account.increase_holders_count(&destination_levels, destination_country)?
        } else {
            vec![]
        };

        if !decreased_counters.is_empty() {
            policy_engine_account.enforce_counters_on_decrement(&decreased_counters)?;
        }

        if !increased_counters.is_empty() {
            policy_engine_account.enforce_counters_on_increment(&increased_counters)?;
        }

        if !increased_counters.is_empty() || !decreased_counters.is_empty() {
            let data = policy_engine_account.try_to_vec()?;
            let len = data.len();
            ctx.accounts.policy_engine_account.data.borrow_mut()[8..8 + len].copy_from_slice(&data);
        }
    }

    // evaluate policies
    policy_engine_account.enforce_policy(
        amount,
        timestamp,
        &source_levels,
        source_country,
        &destination_levels,
        destination_country,
        &source_tracker_account,
        &destination_tracker_account,
        source_balance,
        destination_balance,
        self_transfer,
    )?;


    Ok(())
}
