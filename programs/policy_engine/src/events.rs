use anchor_lang::prelude::*;

#[event]
pub struct TransferEvent {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}

#[event]
pub struct MintEvent {
    pub to: Pubkey,
    pub amount: u64,
    pub issue_time: u64
}


#[event]
pub struct LockEvent {
    pub amount: u64,
    pub release_timestamp: i64,
    pub reason: u64,
    pub reason_string: String,
}

#[event]
pub struct UnlockEvent {
    pub amount: u64,
    pub release_timestamp: i64,
    pub reason: u64,
    pub reason_string: String,
}