use crate::{state::*, IdentityAccount, IdentityRegistryAccount, IdentityRegistryErrors};
use anchor_lang::{prelude::*, solana_program::program_option::COption};
use anchor_spl::{
    token_2022::{freeze_account, FreezeAccount},
    token_interface::{Mint, Token2022, TokenAccount},
};

#[derive(Accounts)]
#[instruction(wallet: Pubkey)]
pub struct AttachWalletToIdentity<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account()]
    pub owner: Signer<'info>,
    #[account(
        has_one = identity_registry,
        has_one = owner
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

    pub system_program: Program<'info, System>,
}


pub fn handler(ctx: Context<AttachWalletToIdentity>, wallet: Pubkey) -> Result<()> {
    require!(ctx.accounts.identity_registry.allow_multiple_wallets, IdentityRegistryErrors::MultipleWalletsNotAllowed);
    ctx.accounts.identity_account.add_wallet()?;
    ctx.accounts.wallet_identity.identity_account = ctx.accounts.identity_account.key();
    ctx.accounts.wallet_identity.wallet = wallet;
    Ok(())
}
