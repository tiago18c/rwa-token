use anchor_lang::{prelude::*, AnchorSerialize};
use identity_registry::IdentityLevel;
use num_enum::IntoPrimitive;
use serde::{Deserialize, Serialize};

use crate::{enforce_identity_filter, enforce_identity_filter2, enforce_transfer_identity_filter, get_total_amount_transferred_in_timeframe, get_total_transactions_in_timeframe, PolicyEngineErrors};

use super::TrackerAccount;

#[derive(
    AnchorDeserialize, AnchorSerialize, Clone, InitSpace, Copy, Debug, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct IdentityFilter {
    pub identity_levels: [u8; 10],         // 10
    pub comparision_type: ComparisionType, // 2
    pub counterparty_filter: CounterpartyFilter, // 2
}

#[repr(u8)]
#[derive(
    IntoPrimitive,
    AnchorDeserialize,
    AnchorSerialize,
    Clone,
    InitSpace,
    Copy,
    Debug,
    Serialize,
    Deserialize,
)]
pub enum ComparisionType {
    Or,
    And,
    Except
}

#[repr(u8)]
#[derive(
    IntoPrimitive,
    AnchorDeserialize,
    AnchorSerialize,
    Clone,
    InitSpace,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
)]
pub enum CounterpartyFilter {
    Sender,
    Receiver,
    Both,
}

#[account()]
#[derive(InitSpace)]
pub struct PolicyEngineAccount {
    /// version
    pub version: u8,
    /// asset mint
    pub asset_mint: Pubkey,
    /// authority of the registry
    pub authority: Pubkey,
    /// policy delegate
    pub delegate: Pubkey,
    /// max timeframe of all the policies
    pub max_timeframe: i64,
    /// enforce policy issuance
    pub enforce_policy_issuance: bool,
    #[max_len(1)]
    /// initial max len
    pub policies: Vec<Policy>,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Policy {
    #[max_len(64)]
    pub hash: String,
    pub identity_filter: IdentityFilter,
    pub policy_type: PolicyType,
}

#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    Clone,
    InitSpace,
    PartialEq,
    Copy,
    Debug,
    Serialize,
    Deserialize,
)]
pub enum PolicyType {
    IdentityApproval,
    TransactionAmountLimit { limit: u64 },
    TransactionAmountVelocity { limit: u64, timeframe: i64 },
    TransactionCountVelocity { limit: u64, timeframe: i64 },
    MaxBalance { limit: u64 },
    MinBalance { limit: u64 },
    TransferPause,
    ForbiddenIdentityGroup,
    ForceFullTransfer,
}

pub fn get_policy_engine_pda(asset_mint: Pubkey) -> Pubkey {
    let (pda, _) = Pubkey::find_program_address(&[&asset_mint.to_bytes()], &crate::ID);
    pda
}

impl PolicyEngineAccount {
    pub const VERSION: u8 = 1;
    pub fn new(&mut self, authority: Pubkey, delegate: Option<Pubkey>, asset_mint: Pubkey, enforce_policy_issuance: bool) {
        self.version = Self::VERSION;
        self.authority = authority;
        self.delegate = delegate.unwrap_or(authority);
        self.asset_mint = asset_mint;
        self.enforce_policy_issuance = enforce_policy_issuance;
    }
    pub fn update_delegate(&mut self, delegate: Pubkey) {
        self.delegate = delegate;
    }

    /// update max timeframe if new value is greater than current
    pub fn update_max_timeframe(&mut self, policy_type: PolicyType) {
        let mut max_timeframe = self.max_timeframe;
        match policy_type {
            PolicyType::TransactionAmountVelocity {
                limit: _,
                timeframe,
            } => {
                if timeframe > max_timeframe {
                    max_timeframe = timeframe;
                }
            }
            PolicyType::TransactionCountVelocity {
                limit: _,
                timeframe,
            } => {
                if timeframe > max_timeframe {
                    max_timeframe = timeframe;
                }
            }
            _ => {}
        }
        self.max_timeframe = max_timeframe;
    }
    
    pub fn hash_policy(
        policy_account: Pubkey,
        policy_type: PolicyType,
        identity_filter: IdentityFilter,
    ) -> String {
        let hash = format!("{:?}{:?}{:?}", policy_account, policy_type, identity_filter);
        sha256::digest(hash.as_bytes())
    }
    
    pub fn attach(
        &mut self,
        policy_account: Pubkey,
        policy_type: PolicyType,
        identity_filter: IdentityFilter,
    ) -> Result<()> {
        let hash = Self::hash_policy(policy_account, policy_type, identity_filter);
        if self.policies.iter().any(|policy| policy.hash == hash) {
            return Err(PolicyEngineErrors::PolicyAlreadyExists.into());
        }
        self.policies.push(Policy {
            hash,
            policy_type,
            identity_filter,
        });
        Ok(())
    }

    pub fn detach(&mut self, hash: String) -> Result<PolicyType> {
        if self.policies.iter().all(|policy| policy.hash != hash) {
            return Err(PolicyEngineErrors::PolicyNotFound.into());
        }
        // remove and return the policy type
        let policy_type = self
            .policies
            .iter()
            .find(|policy| policy.hash == hash)
            .unwrap() // safe to unwrap as we checked the policy exists
            .policy_type;
        self.policies.retain(|policy| policy.hash != hash);
        Ok(policy_type)
    }

    pub fn enforce_policy_issuance(&self, amount: u64, timestamp: i64, identity: &[IdentityLevel], tracker_account: Option<&TrackerAccount>) -> Result<()> {
        for policy in self.policies.iter() {
            match &policy.policy_type {
                PolicyType::IdentityApproval => {
                    enforce_identity_filter2(identity, policy.identity_filter, timestamp)?;
                }
                PolicyType::TransactionAmountLimit { limit } => {
                    if enforce_identity_filter2(identity, policy.identity_filter, timestamp).is_ok()
                        && amount > *limit {
                        return Err(PolicyEngineErrors::TransactionAmountLimitExceeded.into());
                    }
                }
                PolicyType::TransactionAmountVelocity { limit, timeframe } => {
                    if enforce_identity_filter2(identity, policy.identity_filter, timestamp).is_ok() {
                        if let Some(dst_tracker) = tracker_account {
                            let total_amount_transferred = get_total_amount_transferred_in_timeframe(
                                &dst_tracker.transfers, *timeframe, timestamp,
                            );

                            if total_amount_transferred > *limit {
                                return Err(
                                    PolicyEngineErrors::TransactionAmountVelocityExceeded.into()
                                );
                            }
                        }
                    }
                }
                PolicyType::TransactionCountVelocity { limit, timeframe } => {
                    if enforce_identity_filter2(identity, policy.identity_filter, timestamp).is_ok() {
                        if let Some(dst_tracker) = tracker_account {
                            let total_transactions =
                                get_total_transactions_in_timeframe(&dst_tracker.transfers, *timeframe, timestamp);
                            if total_transactions + 1 > *limit {
                                return Err(PolicyEngineErrors::TransactionCountVelocityExceeded.into());
                            }
                        }
                    }
                }
                PolicyType::MaxBalance { limit } => {
                    if enforce_identity_filter2(identity, policy.identity_filter, timestamp).is_ok() {
                        if let Some(dst_tracker) = tracker_account {
                            if dst_tracker.total_amount > *limit {
                                return Err(PolicyEngineErrors::MaxBalanceExceeded.into());
                            }
                        }
                    }
                }
                PolicyType::TransferPause => {
                    if enforce_identity_filter2(identity, policy.identity_filter, timestamp).is_ok() {
                        return Err(PolicyEngineErrors::TransferPaused.into());
                    }
                }
                PolicyType::ForbiddenIdentityGroup => {
                    if enforce_identity_filter2(identity, policy.identity_filter, timestamp).is_ok()
                    {
                        return Err(PolicyEngineErrors::ForbiddenIdentityGroup.into());
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }


    /// enforces different types of policies
    #[inline(never)]
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_policy(
        &self,
        transfer_amount: u64,
        timestamp: i64,
        source_identity: &[IdentityLevel],
        destination_identity: &[IdentityLevel],
        source_balance: u64,
        source_tracker_account: &Option<TrackerAccount>,
        destination_tracker_account: &Option<TrackerAccount>,
        self_transfer: bool,
    ) -> Result<()> {
        for policy in self.policies.iter() {
            match &policy.policy_type {
                PolicyType::IdentityApproval => {
                    enforce_transfer_identity_filter(destination_identity, source_identity, policy.identity_filter, timestamp)?;
                }
                PolicyType::TransactionAmountLimit { limit } => {
                    if enforce_identity_filter(destination_identity, source_identity, policy.identity_filter, timestamp).is_ok()
                        && transfer_amount > *limit {
                        return Err(PolicyEngineErrors::TransactionAmountLimitExceeded.into());
                    }
                }
                PolicyType::TransactionAmountVelocity { limit, timeframe } => {
                    if !self_transfer && enforce_transfer_identity_filter(destination_identity, source_identity, policy.identity_filter, timestamp).is_ok() {
                        if let Some(dst_tracker) = destination_tracker_account {
                            let total_amount_transferred = get_total_amount_transferred_in_timeframe(
                                &dst_tracker.transfers, *timeframe, timestamp,
                            );

                            if total_amount_transferred > *limit {
                                return Err(
                                    PolicyEngineErrors::TransactionAmountVelocityExceeded.into()
                                );
                            }
                        }
                    }
                }
                PolicyType::TransactionCountVelocity { limit, timeframe } => {
                    if !self_transfer && enforce_transfer_identity_filter(destination_identity, source_identity, policy.identity_filter, timestamp).is_ok() {
                        if let Some(dst_tracker) = destination_tracker_account {
                            let total_transactions =
                                get_total_transactions_in_timeframe(&dst_tracker.transfers, *timeframe, timestamp);
                            if total_transactions > *limit {
                                return Err(PolicyEngineErrors::TransactionCountVelocityExceeded.into());
                            }
                        }
                    }
                }
                PolicyType::MaxBalance { limit } => {
                    if !self_transfer && enforce_transfer_identity_filter(destination_identity, source_identity, policy.identity_filter, timestamp).is_ok() {
                        if let Some(dst_tracker) = destination_tracker_account {
                            if dst_tracker.total_amount > *limit {
                                return Err(PolicyEngineErrors::MaxBalanceExceeded.into());
                            }
                        }
                    }
                }
                PolicyType::MinBalance { limit } => {
                    if !self_transfer && enforce_transfer_identity_filter( destination_identity,source_identity, policy.identity_filter, timestamp).is_ok() {
                        if let Some(src_tracker) = source_tracker_account {
                            if src_tracker.total_amount < *limit {
                                return Err(PolicyEngineErrors::MinBalanceExceeded.into());
                            }
                        }
                    }
                }
                PolicyType::TransferPause => {
                    if enforce_transfer_identity_filter( destination_identity, source_identity, policy.identity_filter, timestamp).is_ok() {
                        return Err(PolicyEngineErrors::TransferPaused.into());
                    }
                }
                PolicyType::ForceFullTransfer => {
                    if enforce_transfer_identity_filter(destination_identity, source_identity, policy.identity_filter, timestamp).is_ok()
                        && source_balance != 0
                    {
                        return Err(PolicyEngineErrors::ForceFullTransfer.into());
                    }
                }
                PolicyType::ForbiddenIdentityGroup => {
                    if enforce_transfer_identity_filter(destination_identity, source_identity, policy.identity_filter, timestamp).is_ok()
                    {
                        return Err(PolicyEngineErrors::ForbiddenIdentityGroup.into());
                    }
                }
            }
        }
        Ok(())
    }
}

/// bec of circular dependancy acp's pubkey is hardcoded
pub fn get_asset_controller_account_pda(asset_mint: Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[asset_mint.as_ref()],
        &pubkey!("acpcFrzEYKjVLvZGWueTV8vyDjhu3oKC7sN38QELLan"),
    )
    .0
}
