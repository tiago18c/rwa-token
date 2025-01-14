use crate::{state::*, IdentityAccount, IdentityRegistryAccount, IdentityRegistryErrors};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

#[derive(Accounts)]
#[instruction(wallet: Pubkey)]
pub struct AttachWalletToIdentity<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account()]
    pub authority: Signer<'info>,
    #[account(
        mut,
        has_one = identity_registry,
    )]
    pub identity_account: Box<Account<'info, IdentityAccount>>,

    #[account(
        has_one = asset_mint
    )]
    pub identity_registry: Box<Account<'info, IdentityRegistryAccount>>,

    pub asset_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        seeds = [wallet.key().as_ref(), asset_mint.key().as_ref()],
        payer = payer,
        space = 8 + WalletIdentity::INIT_SPACE,
        bump,
    )]
    pub wallet_identity: Box<Account<'info, WalletIdentity>>,

    /// CHECK: checked with seeds to enforce wallet is not tied to an existing identity
    #[account(seeds = [identity_registry.key().as_ref(), wallet.as_ref()], bump)]
    pub new_wallet_identity_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AttachWalletToIdentity>, wallet: Pubkey) -> Result<()> {
    require!(
        ctx.accounts.identity_registry.allow_multiple_wallets,
        IdentityRegistryErrors::MultipleWalletsNotAllowed
    );
    require!(
        ctx.accounts.authority.key() == ctx.accounts.identity_account.owner
            || ctx.accounts.authority.key() == ctx.accounts.identity_registry.authority,
        IdentityRegistryErrors::UnauthorizedSigner
    );
    require!(
        ctx.accounts.new_wallet_identity_account.data_is_empty(),
        IdentityRegistryErrors::WalletAlreadyInUse
    );
    ctx.accounts.identity_account.add_wallet()?;
    ctx.accounts.wallet_identity.identity_account = ctx.accounts.identity_account.key();
    ctx.accounts.wallet_identity.wallet = wallet;
    Ok(())
}
