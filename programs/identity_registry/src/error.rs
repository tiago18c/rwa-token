use anchor_lang::prelude::*;

#[error_code]
pub enum IdentityRegistryErrors {
    #[msg("Identity level has already been attached to user")]
    LevelAlreadyPresent,
    #[msg("Number of levels that can be attached to user has been exceeded")]
    MaxLevelsExceeded,
    #[msg("Level to be removed not found")]
    LevelNotFound,
    #[msg("Unauthorized signer")]
    UnauthorizedSigner,
    #[msg("Identity limit reached")]
    LimitReached,
    #[msg("Token account is already initialized")]
    TokenAccountAlreadyInitialized,
    #[msg("Identity creation must be enforced for this feature")]
    IdentityCreationRequired,
    #[msg("Multiple wallets are not allowed")]
    MultipleWalletsNotAllowed,
    #[msg("Wallet already in use")]
    WalletAlreadyInUse,
    #[msg("Invalid level")]
    InvalidLevel,
    #[msg("Token account is not empty")]
    TokenAccountNotEmpty,
}
