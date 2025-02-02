#![allow(ambiguous_glob_reexports, clippy::new_ret_no_self)]

pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

pub use error::*;
pub use events::*;
pub use instructions::*;
pub use state::*;
pub use utils::*;

use anchor_lang::prelude::*;

declare_id!("GZsnjqT3c5zbHqsctrJ4EG4rbEfo7ZXyyUG7aDJNmxfA");

#[program]
pub mod identity_registry {

    use super::*;

    /// registry functions
    /// create identity registry
    pub fn create_identity_registry(
        ctx: Context<CreateIdentityRegistry>,
        authority: Pubkey,
        delegate: Option<Pubkey>,
        allow_multiple_wallets: Option<bool>,
    ) -> Result<()> {
        instructions::registry::create::handler(ctx, authority, delegate, allow_multiple_wallets)
    }

    /// delegate identity registry
    pub fn delegate_identity_regsitry(
        ctx: Context<DelegateIdentityRegistry>,
        delegate: Pubkey,
    ) -> Result<()> {
        instructions::registry::delegate::handler(ctx, delegate)
    }

    /// identity functions
    /// create identity account
    pub fn create_identity_account(
        ctx: Context<CreateIdentityAccount>,
        owner: Pubkey,
        level: u8,
        expiry: i64,
        country: u8,
    ) -> Result<()> {
        instructions::account::create::handler(ctx, owner, level, expiry, country)
    }

    /// add level to identity account
    pub fn add_level_to_identity_account(
        ctx: Context<AddLevelToIdentityAccount>,
        levels: Vec<u8>,
        expiries: Vec<i64>,
        enforce_limits: bool,
    ) -> Result<()> {
        instructions::account::add::handler(ctx, levels, expiries, enforce_limits)
    }

    /// add level to identity account
    pub fn refresh_level_to_identity_account(
        ctx: Context<RefreshLevelToIdentityAccount>,
        level: u8,
        expiry: i64,
    ) -> Result<()> {
        instructions::account::refresh::handler(ctx, level, expiry)
    }

    /// remove level from identity account
    pub fn remove_level_from_identity_account(
        ctx: Context<RemoveLevelFromIdentityAccount>,
        levels: Vec<u8>,
        enforce_limits: bool,
    ) -> Result<()> {
        instructions::account::remove::handler(ctx, levels, enforce_limits)
    }

    /// revoke user identity account by closing account
    pub fn revoke_identity_account(
        ctx: Context<RevokeIdentityAccount>,
        owner: Pubkey,
    ) -> Result<()> {
        instructions::account::revoke::handler(ctx, owner)
    }

    /// attach token account to identity account
    pub fn attach_wallet_to_identity(
        ctx: Context<AttachWalletToIdentity>,
        wallet: Pubkey,
    ) -> Result<()> {
        instructions::account::attach_wallet_to_identity::handler(ctx, wallet)
    }

    /// detach token account from identity account
    pub fn detach_wallet_from_identity(ctx: Context<DetachWalletFromIdentity>) -> Result<()> {
        instructions::account::detach_wallet_from_identity::handler(ctx)
    }

    pub fn change_country(
        ctx: Context<ChangeCountry>,
        new_country: u8,
        enforce_limits: bool,
    ) -> Result<()> {
        instructions::account::change_country::handler(ctx, new_country, enforce_limits)
    }
}
