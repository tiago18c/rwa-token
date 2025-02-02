use crate::{state::*, SeizeEvent};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use rwa_utils::get_bump_in_seed_form;
use spl_token_2022::instruction::transfer_checked;

#[derive(Accounts)]
#[instruction()]
#[event_cpi]
pub struct SeizeTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub asset_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        seeds = [asset_mint.key().as_ref()],
        bump,
        constraint = asset_controller.authority == authority.key()
    )]
    pub asset_controller: Box<Account<'info, AssetControllerAccount>>,
    #[account(mut)]
    pub destination_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub source_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Program<'info, Token2022>,
}

impl<'info> SeizeTokens<'info> {
    fn transfer_tokens(
        &self,
        amount: u64,
        signer_seeds: &[&[&[u8]]],
        remaining_accounts: &[AccountInfo<'info>],
    ) -> Result<()> {
        let mut ix = transfer_checked(
            self.token_program.key,
            &self.source_token_account.key(),
            &self.asset_mint.key(),
            &self.destination_token_account.key(),
            &self.asset_controller.key(),
            &[],
            amount,
            self.asset_mint.decimals,
        )?;

        let mut account_infos = vec![
            self.source_token_account.to_account_info(),
            self.asset_mint.to_account_info(),
            self.destination_token_account.to_account_info(),
            self.asset_controller.to_account_info(),
        ];

        account_infos.extend(remaining_accounts.iter().map(|account| account.clone()));
        ix.accounts
            .extend(remaining_accounts.iter().map(|account| AccountMeta {
                pubkey: *account.key,
                is_signer: account.is_signer,
                is_writable: account.is_writable,
            }));

        anchor_lang::solana_program::program::invoke_signed(&ix, &account_infos, signer_seeds)
            .map_err(Into::into)
    }
}

pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, SeizeTokens<'info>>,
    amount: u64,
    reason: String,
) -> Result<()> {
    let asset_mint = ctx.accounts.asset_mint.key();
    let signer_seeds = [
        asset_mint.as_ref(),
        &get_bump_in_seed_form(&ctx.bumps.asset_controller),
    ];
    ctx.accounts
        .transfer_tokens(amount, &[&signer_seeds], ctx.remaining_accounts)?;

    emit_cpi!(SeizeEvent {
        amount,
        reason,
        wallet: ctx.accounts.source_token_account.owner,
        to_wallet: ctx.accounts.destination_token_account.owner,
        mint: ctx.accounts.asset_mint.key(),
    });

    Ok(())
}
