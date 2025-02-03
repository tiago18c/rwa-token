use anchor_lang::prelude::*;

use crate::{PolicyEngineErrors, TrackerAccount};

#[derive(Accounts)]
#[instruction()]
pub struct CloseTrackerAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Verified to be already closed.
    #[account(constraint = identity_account.data_is_empty() @ PolicyEngineErrors::DataIsNotEmpty)]
    pub identity_account: UncheckedAccount<'info>,
    #[account(
        mut,
        close = payer,
        has_one = identity_account
    )]
    pub tracker_account: Box<Account<'info, TrackerAccount>>,
}
