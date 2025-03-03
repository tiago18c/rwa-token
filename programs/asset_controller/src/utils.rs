use anchor_lang::{
    prelude::{Pubkey, Result}, pubkey, solana_program::{program::invoke, system_instruction::transfer}, Lamports
};

use crate::{AccountInfo, Rent, SolanaSysvar};

pub const ASSET_ACCESS_CONTROLLER_ID: Pubkey = pubkey!("3g6fUgMsJQhjPUYxNjEU5PyzGf8DerbDx8p5Qh4yMnRR");


pub fn update_account_lamports_to_minimum_balance<'info>(
    account: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
) -> Result<()> {
    let min_balance = Rent::get()?.minimum_balance(account.data_len());
    if min_balance > account.get_lamports() {
        invoke(
            &transfer(payer.key, account.key, min_balance - account.get_lamports()),
            &[payer, account, system_program],
        )?;
    }
    Ok(())
}
