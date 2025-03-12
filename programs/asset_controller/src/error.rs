use anchor_lang::prelude::*;

#[error_code]
pub enum AssetControllerErrors {
    #[msg("Transfer hasnt been approved for the asset mint")]
    TransferMintNotApproved,
    #[msg("Transfer hasnt been approved for from account")]
    TransferFromNotApproved,
    #[msg("Transfer hasnt been approved for to account")]
    TransferToNotApproved,
    #[msg("Transfer hasnt been approved for the specified amount")]
    TransferAmountNotApproved,
    #[msg("Invalid policy account passed")]
    InvalidPolicyAccount,
    #[msg("Invalid slot for approve account")]
    TransferSlotNotApproved,
    #[msg("Transfer history is full")]
    TransferHistoryFull,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Pda passed in for transfer is wrong")]
    InvalidPdaPassedIn,
    #[msg("Token account is not initialized")]
    TokenAccountNotInitialized,
    #[msg("Token account is already initialized")]
    TokenAccountAlreadyInitialized,
    #[msg("Invalid identity accounts")]
    InvalidIdentityAccounts,
    #[msg("Invalid authority")]
    InvalidAuthority,
}
