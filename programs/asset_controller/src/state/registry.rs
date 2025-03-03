use anchor_lang::prelude::*;

#[account()]
#[derive(InitSpace)]
pub struct AssetControllerAccount {
    pub version: u8,
    /// mint pubkey
    pub asset_mint: Pubkey,
    /// authority has the ability to change delegate, freeze token accounts, etc.
    pub authority: Pubkey,
}

impl AssetControllerAccount {
    pub const VERSION: u8 = 1;
    pub fn new(asset_mint: Pubkey, authority: Pubkey) -> Self {
        Self {
            asset_mint,
            authority,
            version: Self::VERSION,
        }
    }
    pub fn get_pda(asset_mint: Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[asset_mint.as_ref()], &crate::id()).0
    }
}

#[event]
pub struct AssetMetadataEvent {
    pub mint: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub uri: Option<String>,
    pub decimals: Option<u8>,
}

#[event]
pub struct ExtensionMetadataEvent {
    pub address: String,
    pub extension_type: u8,
    pub metadata: Vec<u8>,
}
