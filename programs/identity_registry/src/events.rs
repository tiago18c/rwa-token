use anchor_lang::prelude::*;

#[event]
pub struct CreatedIdentityEvent {
    pub identity: Pubkey,
    pub mint: Pubkey,
    pub kind: u8,
    pub sender: Pubkey,
    pub owner: Pubkey,
}

#[event]
pub struct AddLevelsToIdentityEvent {
    pub identity: Pubkey,
    pub mint: Pubkey,
    pub levels: Vec<u8>,
    pub expiries: Vec<i64>,
    pub sender: Pubkey,
}

#[event]
pub struct RemoveLevelsFromIdentityEvent {
    pub identity: Pubkey,
    pub mint: Pubkey,
    pub levels: Vec<u8>,
    pub sender: Pubkey,
}

#[event]
pub struct AttachWalletToIdentityEvent {
    pub identity: Pubkey,
    pub mint: Pubkey,
    pub wallet: Pubkey,
    pub sender: Pubkey,
}

#[event]
pub struct DetachWalletFromIdentityEvent {
    pub identity: Pubkey,
    pub mint: Pubkey,
    pub wallet: Pubkey,
    pub sender: Pubkey,
}

#[event]
pub struct RemovedIdentityEvent {
    pub identity: Pubkey,
    pub mint: Pubkey,
    pub kind: u8,
    pub sender: Pubkey,
    pub owner: Pubkey,
}

#[event]
pub struct ChangeCountryEvent {
    pub identity: Pubkey,
    pub mint: Pubkey,
    pub country: u8,
    pub sender: Pubkey,
}
