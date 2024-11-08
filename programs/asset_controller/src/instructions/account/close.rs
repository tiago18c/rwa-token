use anchor_lang::{prelude::*, solana_program::program_option::COption};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{close_account, CloseAccount, Mint, Token2022, TokenAccount},
};
use identity_registry::{program::IdentityRegistry, IdentityAccount, IdentityRegistryAccount};
use rwa_utils::get_bump_in_seed_form;

use crate::AssetControllerAccount;

#[derive(Accounts)]
#[instruction()]
pub struct CloseTokenAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account()]
    pub owner: Signer<'info>,
    #[account()]
    pub asset_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = asset_mint,
        associated_token::authority = owner
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


pub fn handler(ctx: Context<CloseTokenAccount>) -> Result<()> {
    
    let accounts = CloseAccount {
        authority: ctx.accounts.owner.to_account_info(),
        account: ctx.accounts.token_account.to_account_info(),
        destination: ctx.accounts.payer.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts);
    close_account(cpi_ctx)?;
    Ok(())
}
