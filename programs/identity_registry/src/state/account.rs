use anchor_lang::prelude::*;

use crate::IdentityRegistryErrors;

#[account()]
#[derive(InitSpace)]
pub struct IdentityAccount {
    /// version of the account
    pub version: u8,
    /// identity registry to which the account belongs
    pub identity_registry: Pubkey,
    /// owner of the identity account
    pub owner: Pubkey,

    pub num_wallets: u16,
    // identity levels corresponding to the user
    #[max_len(1)] // initial length is 1
    pub levels: Vec<IdentityLevel>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace, Debug)]
pub struct IdentityLevel {
    pub level: u8,
    pub expiry: i64,
}

impl IdentityAccount {
    pub const VERSION: u8 = 1;
    pub fn new(&mut self, owner: Pubkey, identity_registry: Pubkey, level: u8, expiry: i64) {
        self.identity_registry = identity_registry;
        self.owner = owner;
        self.version = Self::VERSION;
        self.levels = vec![IdentityLevel { level, expiry }];
    }

    pub fn add_level(&mut self, level: u8, expiry: i64) -> Result<()> {
        if self.levels.iter().any(|l| l.level == level) {
            return Err(IdentityRegistryErrors::LevelAlreadyPresent.into());
        }
        self.levels.push(IdentityLevel { level, expiry });
        Ok(())
    }

    pub fn remove_level(&mut self, level: u8) -> Result<()> {
        let index = self.levels.iter().position(|l| l.level == level);
        if index.is_none() {
            return Err(IdentityRegistryErrors::LevelNotFound.into());
        }
        self.levels.swap_remove(index.unwrap());
        Ok(())
    }

    pub fn refresh_level(&mut self, level: u8, expiry: i64) -> Result<()> {
        let index = self.levels.iter().position(|l| l.level == level);
        if index.is_none() {
            return Err(IdentityRegistryErrors::LevelNotFound.into());
        }
        self.levels[index.unwrap()].expiry = expiry;
        Ok(())
    }

    pub fn add_wallet(&mut self) -> Result<()> {
        self.num_wallets += 1;
        Ok(())
    }

    pub fn remove_wallet(&mut self) -> Result<()> {
        self.num_wallets -= 1;
        Ok(())
    }
}
