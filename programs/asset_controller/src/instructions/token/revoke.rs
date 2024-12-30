use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{burn, Burn},
    token_interface::{Mint, Token2022, TokenAccount},
};
use rwa_utils::get_bump_in_seed_form;

#[derive(Accounts)]
#[instruction()]
pub struct RevokeTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub asset_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        seeds = [asset_mint.key().as_ref()],
        bump,
        constraint = asset_controller.authority == authority.key()
    )]
    pub asset_controller: Box<Account<'info, AssetControllerAccount>>,
    #[account(mut)]
    pub revoke_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> RevokeTokens<'info> {

    fn burn_tokens(&self, amount: u64, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let accounts = Burn {
            mint: self.asset_mint.to_account_info(),
            authority: self.asset_controller.to_account_info(),
            from: self.revoke_token_account.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        burn(cpi_ctx, amount)?;
        Ok(())
    }
}

pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, RevokeTokens<'info>>,
    amount: u64,
) -> Result<()> {
    let asset_mint = ctx.accounts.asset_mint.key();
    let signer_seeds = [
        asset_mint.as_ref(),
        &get_bump_in_seed_form(&ctx.bumps.asset_controller),
    ];
    ctx.accounts.burn_tokens(amount, &[&signer_seeds])?;
    Ok(())
}
