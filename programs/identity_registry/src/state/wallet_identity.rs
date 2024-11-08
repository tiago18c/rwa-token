use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct WalletIdentity {
    pub identity_account: Pubkey,
    pub wallet: Pubkey,
}
