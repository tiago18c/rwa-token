use anchor_lang::{prelude::*, AnchorSerialize};
use identity_registry::IdentityLevel;
use num_enum::IntoPrimitive;
use serde::{Deserialize, Serialize};

use crate::{enforce_identity_filter, enforce_transfer_identity_filter, get_total_amount_transferred_in_timeframe, get_total_transactions_in_timeframe, PolicyEngineErrors};

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
    #[max_len(0)]
    pub counters: Vec<Counter>,
    #[max_len(0)]
    pub counter_limits: Vec<CounterLimit>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, Debug)]
pub struct Counter {
    pub value: u64,
    pub id: u8,
    pub identity_filter: IdentityFilter,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, Debug)]
pub enum CounterLimit {
    HoldersLimit { max: u64, min: u64, counter_id: u8 },
    GroupedHoldersLimit { max: u64, min: u64, #[max_len(0)] counters: Vec<u8> },
    PercentageLimit { higher_counter_id: u8, lower_counter_id: u8, min_percentage: u8, max_percentage: u8 },
}

impl CounterLimit {
    pub fn get_space(&self) -> usize {
        match self {
            CounterLimit::GroupedHoldersLimit { max: _, min: _, counters } => Counter::INIT_SPACE + counters.len(),
            _ => CounterLimit::INIT_SPACE,
        }
    }

}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, Debug)]
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
    MinMaxBalance { min: u64, max: u64 },
    TransferPause,
    ForbiddenIdentityGroup,
    ForceFullTransfer,
    BlockFlowbackEndTime { time: i64, target_level: u8 },
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
    pub fn update_max_timeframe(&mut self, policy_type: &PolicyType) {
        let mut max_timeframe = self.max_timeframe;
        match policy_type {
            PolicyType::TransactionAmountVelocity {
                limit: _,
                timeframe,
            } => {
                if *timeframe > max_timeframe {
                    max_timeframe = *timeframe;
                }
            }
            PolicyType::TransactionCountVelocity {
                limit: _,
                timeframe,
            } => {
                if *timeframe > max_timeframe {
                    max_timeframe = *timeframe;
                }
            }
            _ => {}
        }
        self.max_timeframe = max_timeframe;
    }
    
    pub fn hash_policy(
        policy_account: Pubkey,
        policy_type: &PolicyType,
        identity_filter: &IdentityFilter,
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
        let hash = Self::hash_policy(policy_account, &policy_type, &identity_filter);
        if self.policies.iter().any(|policy| policy.hash == hash) {
            return Err(PolicyEngineErrors::PolicyAlreadyExists.into());
        }
        self.policies.push(Policy { hash, identity_filter, policy_type });
        Ok(())
    }

    pub fn detach(&mut self, hash: String) -> Result<PolicyType> {
        if let Some(index) = self.policies.iter().position(|policy| policy.hash == hash) {
            return Ok(self.policies.remove(index).policy_type)
        }
        Err(PolicyEngineErrors::PolicyNotFound.into())
    }

    pub fn enforce_policy_issuance(&self, amount: u64, timestamp: i64, identity: &[IdentityLevel], tracker_account: Option<&TrackerAccount>) -> Result<()> {
        for policy in self.policies.iter() {
            match &policy.policy_type {
                PolicyType::IdentityApproval => {
                    enforce_identity_filter(identity, policy.identity_filter, timestamp)?;
                }
                PolicyType::TransactionAmountLimit { limit } => {
                    if enforce_identity_filter(identity, policy.identity_filter, timestamp).is_ok()
                        && amount > *limit {
                        return Err(PolicyEngineErrors::TransactionAmountLimitExceeded.into());
                    }
                }
                PolicyType::TransactionAmountVelocity { limit, timeframe } => {
                    if enforce_identity_filter(identity, policy.identity_filter, timestamp).is_ok() {
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
                    if enforce_identity_filter(identity, policy.identity_filter, timestamp).is_ok() {
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
                    if enforce_identity_filter(identity, policy.identity_filter, timestamp).is_ok() {
                        if let Some(dst_tracker) = tracker_account {
                            if dst_tracker.total_amount > *limit {
                                return Err(PolicyEngineErrors::MaxBalanceExceeded.into());
                            }
                        }
                    }
                }
                PolicyType::MinBalance { limit } => {
                    if enforce_identity_filter(identity, policy.identity_filter, timestamp).is_ok() {
                        if let Some(dst_tracker) = tracker_account {
                            if dst_tracker.total_amount < *limit {
                                return Err(PolicyEngineErrors::MinBalanceExceeded.into());
                            }
                        }
                    }
                }
                PolicyType::MinMaxBalance { min, max } => {
                    if enforce_identity_filter(identity, policy.identity_filter, timestamp).is_ok() {
                        if let Some(dst_tracker) = tracker_account {
                            if dst_tracker.total_amount > *max {
                                return Err(PolicyEngineErrors::MaxBalanceExceeded.into());
                            }
                            if dst_tracker.total_amount < *min {
                                return Err(PolicyEngineErrors::MinBalanceExceeded.into());
                            }
                        }
                    }
                }
                PolicyType::TransferPause => {
                    if enforce_identity_filter(identity, policy.identity_filter, timestamp).is_ok() {
                        return Err(PolicyEngineErrors::TransferPaused.into());
                    }
                }
                PolicyType::ForbiddenIdentityGroup => {
                    if enforce_identity_filter(identity, policy.identity_filter, timestamp).is_ok()
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
        _source_tracker_account: &Option<Box<TrackerAccount>>,
        destination_tracker_account: &Option<Box<TrackerAccount>>,
        source_balance: u64,
        destination_balance: u64,
        self_transfer: bool,
    ) -> Result<()> {
        for policy in self.policies.iter() {
            match &policy.policy_type {
                PolicyType::IdentityApproval => {
                    enforce_transfer_identity_filter(destination_identity, source_identity, policy.identity_filter, timestamp)?;
                }
                PolicyType::TransactionAmountLimit { limit } => {
                    if enforce_transfer_identity_filter(destination_identity, source_identity, policy.identity_filter, timestamp).is_ok()
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
                        if destination_balance > *limit {
                            return Err(PolicyEngineErrors::MaxBalanceExceeded.into());
                        }
                    }
                }
                PolicyType::MinBalance { limit } => {
                    if !self_transfer && enforce_transfer_identity_filter( destination_identity,source_identity, policy.identity_filter, timestamp).is_ok() {
                        if source_balance < *limit {
                            return Err(PolicyEngineErrors::MinBalanceExceeded.into());
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
                PolicyType::MinMaxBalance { min, max } => {
                    if enforce_transfer_identity_filter(destination_identity, source_identity, policy.identity_filter, timestamp).is_ok() {
                        if source_balance < *min || destination_balance < *min {
                            return Err(PolicyEngineErrors::MinBalanceExceeded.into());
                        }
                        if source_balance > *max || destination_balance > *max {
                            return Err(PolicyEngineErrors::MaxBalanceExceeded.into());
                        }
                    }
                }
                PolicyType::BlockFlowbackEndTime { time, target_level } => {
                    if enforce_identity_filter(source_identity, policy.identity_filter, timestamp).is_ok() {
                        if  (*time == 0 || *time > timestamp) && destination_identity.iter().any(|level| level.level == *target_level && level.expiry > timestamp) {
                            return Err(PolicyEngineErrors::Flowback.into());
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn decrease_holders_count(&mut self, identity: &[IdentityLevel]) -> Result<Vec<u8>> {
        let mut changed_counters = Vec::new();
        for counter in self.counters.iter_mut() {
            if enforce_identity_filter(identity, counter.identity_filter, 0).is_ok() {
                counter.value -= 1;
                changed_counters.push(counter.id);
            }
        }

        Ok(changed_counters)
    }

    pub fn increase_holders_count(&mut self, identity: &[IdentityLevel]) -> Result<Vec<u8>> {
        let mut changed_counters = Vec::new();
        for counter in self.counters.iter_mut() {
            if enforce_identity_filter(identity, counter.identity_filter, 0).is_ok() {
                counter.value += 1;
                changed_counters.push(counter.id);
            }
        }

        Ok(changed_counters)
    }

    pub fn enforce_counters_on_increment(&self, incremented_counters: &[u8]) -> Result<()> {
        for counter_limit in self.counter_limits.iter() {
            match counter_limit {
                CounterLimit::HoldersLimit { max, min: _, counter_id } => {
                    if incremented_counters.contains(&counter_id) {
                        let counter = self.counters.iter().find(|counter| counter.id == *counter_id).unwrap();
                        if *max < counter.value {
                            return Err(PolicyEngineErrors::HoldersLimitExceeded.into());
                        }
                    }
                }
                CounterLimit::GroupedHoldersLimit { max, min: _, counters } => {
                    if incremented_counters.iter().any(|id| counters.contains(id)) {
                        for counter_id in counters.iter() {
                            let counter = self.counters.iter().find(|counter| counter.id == *counter_id).unwrap();
                            if *max < counter.value {
                                return Err(PolicyEngineErrors::HoldersLimitExceeded.into());
                            }
                        }
                    }
                }
                CounterLimit::PercentageLimit { higher_counter_id, lower_counter_id, min_percentage, max_percentage } => {
                    if incremented_counters.contains(&higher_counter_id) || incremented_counters.contains(&lower_counter_id) {
                        let higher_counter = self.counters.iter().find(|counter| counter.id == *higher_counter_id).unwrap().value;
                        let lower_counter = self.counters.iter().find(|counter| counter.id == *lower_counter_id).unwrap().value;

                        let percent = lower_counter * 100 / higher_counter;

                        if *max_percentage < 100 && percent > *max_percentage as u64 {
                            return Err(PolicyEngineErrors::PercentageLimitExceeded.into());
                        }
                        if *min_percentage > 0 && percent < *min_percentage as u64 {
                            return Err(PolicyEngineErrors::PercentageLimitExceeded.into());
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn enforce_counters_on_decrement(&self, decremented_counters: &[u8]) -> Result<()> {
        for counter_limit in self.counter_limits.iter() {
            match counter_limit {
                CounterLimit::HoldersLimit { max: _, min, counter_id } => {
                    if decremented_counters.contains(&counter_id) {
                        let counter = self.counters.iter().find(|counter| counter.id == *counter_id).unwrap();
                        if *min > counter.value {
                            return Err(PolicyEngineErrors::HoldersLimitExceeded.into());
                        }
                    }
                }
                CounterLimit::GroupedHoldersLimit { max: _, min, counters } => {
                    for counter_id in counters.iter() {
                        if decremented_counters.contains(counter_id) {
                            let counter = self.counters.iter().find(|counter| counter.id == *counter_id).unwrap();
                            if *min > counter.value {
                                return Err(PolicyEngineErrors::HoldersLimitExceeded.into());
                            }
                        }
                    }
                }
                CounterLimit::PercentageLimit { higher_counter_id, lower_counter_id, min_percentage, max_percentage } => {
                    if decremented_counters.contains(&higher_counter_id) || decremented_counters.contains(&lower_counter_id) {
                        let higher_counter = self.counters.iter().find(|counter| counter.id == *higher_counter_id).unwrap().value;
                        let lower_counter = self.counters.iter().find(|counter| counter.id == *lower_counter_id).unwrap().value;

                        let percent = lower_counter * 100 / higher_counter;

                        if *max_percentage < 100 && percent > *max_percentage as u64 {
                            return Err(PolicyEngineErrors::PercentageLimitExceeded.into());
                        }
                        if *min_percentage > 0 && percent < *min_percentage as u64 {
                            return Err(PolicyEngineErrors::PercentageLimitExceeded.into());
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn update_counters_on_levels_change(&mut self, previous_levels: &[IdentityLevel], new_levels: &[IdentityLevel]) -> Result<()> {
        let mut incremented_counters = Vec::new();
        let mut decremented_counters = Vec::new();
        for counter in self.counters.iter_mut() {
            let prev_match = enforce_identity_filter(previous_levels, counter.identity_filter, 0).is_ok();

            let new_match = enforce_identity_filter(new_levels, counter.identity_filter, 0).is_ok();

            if prev_match && !new_match {
                counter.value -= 1;
                decremented_counters.push(counter.id);
            } else if !prev_match && new_match {
                counter.value += 1;
                incremented_counters.push(counter.id);
            }
        }

        self.enforce_counters_on_increment(&incremented_counters)?;
        self.enforce_counters_on_decrement(&decremented_counters)?;

        Ok(())
    }

    pub fn enforce_policy_on_levels_change(&self, new_levels: &[IdentityLevel], timestamp: i64, balance: u64, enforce_limits: bool) -> Result<()> {
        for policy in self.policies.iter() {
            match &policy.policy_type {
                PolicyType::MinMaxBalance { min, max } => {
                    if enforce_limits && enforce_identity_filter(new_levels, policy.identity_filter, timestamp).is_ok() {
                        if balance < *min || balance > *max {
                            return Err(PolicyEngineErrors::MinMaxBalanceExceeded.into());
                        }
                    }
                }
                PolicyType::MaxBalance { limit } => {
                    if enforce_limits && enforce_identity_filter(new_levels, policy.identity_filter, timestamp).is_ok() {
                        if balance > *limit {
                            return Err(PolicyEngineErrors::MaxBalanceExceeded.into());
                        }
                    }
                }
                PolicyType::MinBalance { limit } => {
                    if enforce_limits && enforce_identity_filter(new_levels, policy.identity_filter, timestamp).is_ok() {
                        if balance < *limit {
                            return Err(PolicyEngineErrors::MinBalanceExceeded.into());
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn update_and_enforce_policy_and_levels_on_levels_change(&mut self, previous_levels: &[IdentityLevel], new_levels: &[IdentityLevel], timestamp: i64, balance: u64, enforce_limits: bool) -> Result<()> {
        self.update_counters_on_levels_change(previous_levels, new_levels)?;
        self.enforce_policy_on_levels_change(new_levels, timestamp, balance, enforce_limits)
    }


    pub fn update_counters(&mut self, removed_counters: Vec<u8>, added_counters: Vec<Counter>) -> Result<()> {
        self.counters.retain(|c| !removed_counters.contains(&c.id));
        self.counters.extend(added_counters);
        Ok(())
    }

    pub fn update_counter_limits(&mut self, removed_counter_limits: Vec<u8>, added_counter_limits: Vec<CounterLimit>) -> Result<i32> {

        let mut space: i32 = -removed_counter_limits.iter().rev().map(|id| {
            self.counter_limits.remove(*id as usize).get_space() as i32
        }).sum::<i32>();
        
        space += added_counter_limits.iter().map(|limit| {
            limit.get_space() as i32
        }).sum::<i32>();

        self.counter_limits.extend(added_counter_limits);

        Ok(space)
    }
}

/// bec of circular dependancy acp's pubkey is hardcoded
pub fn get_asset_controller_account_pda(asset_mint: Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[asset_mint.as_ref()],
        &pubkey!("2fJZj6eLMvdGkHTePvbXGjh9x1hHSMvkBPhTcAAPvBMa"),
    )
    .0
}
