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

declare_id!("po1cPf1eyUJJPqULw4so3T4JU9pdFn83CDyuLEKFAau");

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
        additional_levels: Option<Vec<u8>>,
    ) -> Result<()> {
        instructions::engine::attach::handler(ctx, identity_filter, policy_type, additional_levels)
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

    pub fn enforce_policy_issuance(ctx: Context<EnforcePolicyIssuanceAccounts>, amount: u64) -> Result<()> {
        instructions::issue::handler(ctx, amount)
    }
}
