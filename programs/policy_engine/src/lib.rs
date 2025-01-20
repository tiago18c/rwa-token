#![allow(ambiguous_glob_reexports, clippy::new_ret_no_self)]

pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

pub use error::*;
pub use instructions::*;
pub use state::*;
pub use utils::*;

use anchor_lang::prelude::*;

declare_id!("FsE8mCJyvgMzqJbfHbJQm3iuf3cRZC6n2vZi1Q8rQCy2");

#[program]
#[allow(deprecated)]
pub mod policy_engine {

    use super::*;

    /// create a policy registry
    pub fn create_policy_engine(
        ctx: Context<CreatePolicyEngine>,
        authority: Pubkey,
        delegate: Option<Pubkey>,
        enforce_policy_issuance: Option<bool>,
    ) -> Result<()> {
        instructions::engine::create::handler(ctx, authority, delegate, enforce_policy_issuance)
    }

    /// policies
    /// attach a policy
    pub fn attach_to_policy_engine(
        ctx: Context<AttachToPolicyEngine>,
        identity_filter: IdentityFilter,
        policy_type: PolicyType,
    ) -> Result<()> {
        instructions::engine::attach::handler(ctx, identity_filter, policy_type)
    }

    /// remove policy
    pub fn detach_from_policy_engine(
        ctx: Context<DetachFromPolicyEngine>,
        hash: String,
    ) -> Result<()> {
        instructions::engine::detach::handler(ctx, hash)
    }

    /// create tracker account
    pub fn create_tracker_account(ctx: Context<CreateTrackerAccount>) -> Result<()> {
        instructions::tracker::handler(ctx)
    }

    /// execute transfer hook
    #[interface(spl_transfer_hook_interface::execute)]
    pub fn execute_transaction(ctx: Context<ExecuteTransferHook>, amount: u64) -> Result<()> {
        instructions::execute::handler(ctx, amount)
    }

    pub fn enforce_policy_issuance(
        ctx: Context<EnforcePolicyIssuanceAccounts>,
        amount: u64,
        issuance_timestamp: i64,
    ) -> Result<()> {
        instructions::issue::handler(ctx, amount, issuance_timestamp)
    }

    pub fn update_counters_on_burn(
        ctx: Context<UpdateCountersOnBurnAccounts>,
        amount: u64,
    ) -> Result<()> {
        instructions::update_counters_on_burn::handler(ctx, amount)
    }

    pub fn enforce_policy_on_levels_change(
        ctx: Context<EnforcePolicyOnLevelsChange>,
        new_levels: NewLevelsArgs,
        new_country: u8,
        enforce_limits: bool,
    ) -> Result<()> {
        instructions::enforce_policy_on_levels_change::handler(
            ctx,
            new_levels,
            new_country,
            enforce_limits,
        )
    }

    pub fn change_counters(
        ctx: Context<ChangeCounters>,
        removed_counters: Vec<u8>,
        added_counters: Vec<Counter>,
    ) -> Result<()> {
        instructions::engine::change_counters::handler(ctx, removed_counters, added_counters)
    }

    pub fn change_counter_limits(
        ctx: Context<ChangeCounterLimits>,
        removed_counter_limits: Vec<u8>,
        added_counter_limits: Vec<CounterLimit>,
    ) -> Result<()> {
        instructions::engine::change_counter_limits::handler(
            ctx,
            removed_counter_limits,
            added_counter_limits,
        )
    }

    pub fn change_mapping(
        ctx: Context<ChangeMapping>,
        mapping_source: Vec<u8>,
        mapping_value: Vec<u8>,
    ) -> Result<()> {
        instructions::engine::change_mapping::handler(ctx, mapping_source, mapping_value)
    }

    pub fn change_issuance_policies(
        ctx: Context<ChangeIssuancePolicies>,
        issuance_policies: IssuancePolicies,
    ) -> Result<()> {
        instructions::engine::change_issuance_policies::handler(ctx, issuance_policies)
    }

    pub fn set_counters(
        ctx: Context<SetCounters>,
        changed_counters: Vec<u8>,
        values: Vec<u64>,
    ) -> Result<()> {
        instructions::engine::set_counters::handler(ctx, changed_counters, values)
    }
}
