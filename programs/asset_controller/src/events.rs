use anchor_lang::prelude::*;

#[event]
pub struct BurnEvent {
    pub amount: u64,
    pub reason: String,
    pub wallet: Pubkey,
}


#[event]
pub struct SeizeEvent {
    pub amount: u64,
    pub reason: String,
    pub wallet: Pubkey,
}

#[event]
pub struct RevokeEvent {
    pub amount: u64,
    pub reason: String,
    pub wallet: Pubkey,
}
