use anchor_lang::prelude::*;

use crate::{PolicyEngineErrors, Side, Transfer};

pub const MAX_TRANSFER_HISTORY: usize = 25;

#[account()]
#[derive(InitSpace)]
pub struct TrackerAccount {
    pub version: u8,
    // corresponding asset mint
    pub asset_mint: Pubkey,
    // identity account of the owner of the policy account
    pub identity_account: Pubkey,
    // transfer amounts
    // this is not a realloc field because we dont have a mutable account during transfers to transfer the required amount
    #[max_len(MAX_TRANSFER_HISTORY)]
    // MAX_TRANSFER_HISTORY is the max number of transfers we want to store
    pub transfers: Vec<Transfer>,

    pub total_amount: u64,
}

impl TrackerAccount {
    pub const VERSION: u8 = 1;
    pub fn new(&mut self, asset_mint: Pubkey, owner: Pubkey) {
        self.version = Self::VERSION;
        self.asset_mint = asset_mint;
        self.identity_account = owner;
    }
    /// for all timestamps, if timestamp is older than timestamp - max_timeframe. remove it,
    #[inline(never)]
    pub fn update_transfer_history(
        &mut self,
        amount: u64,
        timestamp: i64,
        max_timeframe: i64,
        side: Side,
    ) -> Result<()> {
        self.total_amount = if side != Side::Sell {
            self.total_amount + amount
        } else {
            self.total_amount - amount
        };

        if max_timeframe == 0 {
            // if max_timeframe is 0, we dont need to track any history
            return Ok(());
        }
        let min_timestamp = timestamp - max_timeframe;
        self.transfers
            .retain(|transfer| transfer.timestamp >= min_timestamp);
        self.transfers.push(Transfer {
            amount,
            timestamp,
            side,
        });
        // return error if the transfer history is too large
        if self.transfers.len() > MAX_TRANSFER_HISTORY {
            return Err(PolicyEngineErrors::TransferHistoryFull.into());
        }
        Ok(())
    }

    pub fn update_balance_mint(&mut self, amount: u64) -> Result<()> {
        self.total_amount += amount;
        Ok(())
    }

    pub fn update_balance_burn(&mut self, amount: u64) -> Result<()> {
        self.total_amount -= amount;
        Ok(())
    }
}
