use crate::{state::*, PolicyEngineErrors};
use anchor_lang::prelude::*;
use anchor_spl::token_2022::spl_token_2022::extension::StateWithExtensions;
use anchor_spl::token_2022::spl_token_2022::extension::BaseStateWithExtensions;
use anchor_spl::token_2022::spl_token_2022::extension::transfer_hook::TransferHookAccount;
use anchor_spl::token_2022::spl_token_2022::state::Account as Token2022Account;
use identity_registry::IdentityLevel;
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};


pub fn enforce_identity_filter(identity: &[IdentityLevel], identity_filter: IdentityFilter, timestamp: i64) -> Result<()> {
    match identity_filter.comparision_type {
        ComparisionType::Or => {
            // if any level is in the identities array, return Ok
            for level in identity_filter.identity_levels.iter() {
                if *level > 0 && identity.iter().any(|l| l.expiry > timestamp && l.level == *level) {
                    return Ok(());
                }
            }
            Err(PolicyEngineErrors::IdentityFilterFailed.into())
        }
        ComparisionType::And => {
            // if all levels are in the identities array, return Ok
            for level in identity_filter.identity_levels.iter() {
                if *level > 0 && !identity.iter().any(|l| l.expiry > timestamp && l.level == *level) {
                    return Err(PolicyEngineErrors::IdentityFilterFailed.into());
                }
            }
            Ok(())
        }
        ComparisionType::Except => {
            // if any level is in the identities array, return Err
            for level in identity_filter.identity_levels.iter() {
                if *level > 0 && identity.iter().any(|l| l.expiry > timestamp && l.level == *level) {
                    return Err(PolicyEngineErrors::IdentityFilterFailed.into());
                }
            }
            Ok(())
        }
    }
}

pub fn enforce_transfer_identity_filter(destination_identity: &[IdentityLevel], source_identity: &[IdentityLevel], identity_filter: IdentityFilter, timestamp: i64) -> Result<()> {
    match (identity_filter.comparision_type, identity_filter.counterparty_filter) {
        (ComparisionType::Or, CounterpartyFilter::Both) => {
            enforce_identity_filter(destination_identity, identity_filter, timestamp)
            .or(enforce_identity_filter(source_identity, identity_filter, timestamp))
        }
        (_, CounterpartyFilter::Receiver) => {
            enforce_identity_filter(destination_identity, identity_filter, timestamp)
        }
        (_, CounterpartyFilter::Sender) => {
            enforce_identity_filter(source_identity, identity_filter, timestamp)
        }
        (_, CounterpartyFilter::Both) => 
        {
            enforce_identity_filter(destination_identity, identity_filter, timestamp)?;
            enforce_identity_filter(source_identity, identity_filter, timestamp)
        }
    }
}

pub fn get_total_amount_transferred_in_timeframe(
    transfers: &Vec<Transfer>,
    timeframe: i64,
    timestamp: i64,
) -> u64 {
    let mut total_amount_transferred = 0;
    let min_timestamp = timestamp - timeframe;
    for transfer in transfers {
        if transfer.timestamp >= min_timestamp {
            total_amount_transferred += transfer.amount;
        }
    }
    total_amount_transferred
}

pub fn get_total_transactions_in_timeframe(
    transfers: &Vec<Transfer>,
    timeframe: i64,
    timestamp: i64,
) -> u64 {
    let mut total_transactions = 0;
    let min_timestamp = timestamp - timeframe;
    for transfer in transfers {
        if transfer.timestamp >= min_timestamp {
            total_transactions += 1;
        }
    }
    total_transactions
}

#[derive(InitSpace, AnchorDeserialize, AnchorSerialize, Copy, Clone, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(InitSpace, AnchorDeserialize, AnchorSerialize, Copy, Clone)]
pub struct Transfer {
    pub amount: u64,
    pub timestamp: i64,
    pub side: Side,
}

pub fn assert_is_transferring(account: &AccountInfo) -> Result<()> {
    let account_data = account.try_borrow_data()?;
    let token_account = StateWithExtensions::<Token2022Account>::unpack(&account_data)?;
    let account_extension = token_account.get_extension::<TransferHookAccount>()?;
 
    // can assume if its not transferring, it wasn't called by token22
    if !bool::from(account_extension.transferring) {
        return Err(PolicyEngineErrors::InvalidCpiTransferProgram.into());
    }
 
    Ok(())
}

#[inline(never)]
pub fn verify_pda(address: Pubkey, seeds: &[&[u8]], program_id: &Pubkey) -> Result<()> {
    let (pda, _) = Pubkey::find_program_address(seeds, program_id);
    if pda != address {
        return Err(PolicyEngineErrors::InvalidPdaPassedIn.into());
    }
    Ok(())
}

pub fn get_meta_list_size() -> Result<usize> {
    Ok(ExtraAccountMetaList::size_of(get_extra_account_metas()?.len()).unwrap())
}

pub fn get_extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
    Ok(vec![
        // policy engine account
        ExtraAccountMeta::new_with_seeds(&[Seed::AccountKey { index: 1 }], false, true)?,
        // identity program
        ExtraAccountMeta::new_with_pubkey(&identity_registry::id(), false, false)?,
        // identity registry account
        ExtraAccountMeta::new_external_pda_with_seeds(
            6,
            &[Seed::AccountKey { index: 1 }],
            false,
            false,
        )?,
        // destination identity account
        ExtraAccountMeta::new_external_pda_with_seeds(
            6,
            &[
                Seed::AccountKey { index: 7 },
                Seed::AccountData {
                    // to pubkey
                    account_index: 2, // to token account
                    data_index: 32,
                    length: 32,
                },
            ],
            false,
            false,
        )?,
        // source identity account
        ExtraAccountMeta::new_external_pda_with_seeds(
            6,
            &[
                Seed::AccountKey { index: 7 },
                Seed::AccountData {
                    account_index: 0,
                    data_index: 32,
                    length: 32,
                },
            ],
            false,
            false,
        )?,
        // destination tracker account
        ExtraAccountMeta::new_with_seeds(
            &[
                Seed::AccountKey { index: 1 },
                Seed::AccountKey { index: 8 },
            ],
            false,
            true,
        )?,
        // source tracker account
        ExtraAccountMeta::new_with_seeds(
            &[
                Seed::AccountKey { index: 1 },
                Seed::AccountKey { index: 9 },
            ],
            false,
            true,
        )?,
        // destination wallet identity
        ExtraAccountMeta::new_external_pda_with_seeds(
            6,
            &[
                Seed::AccountData {
                    account_index: 2,
                    data_index: 32,
                    length: 32,
                },
                Seed::AccountKey { index: 1 },
            ],
            false,
            false,
        )?,
        // source wallet identity
        ExtraAccountMeta::new_external_pda_with_seeds(
            6,
            &[
                Seed::AccountData {
                    account_index: 0,
                    data_index: 32,
                    length: 32,
                },
                Seed::AccountKey { index: 1 },
            ],
            false,
            false,
        )?,
    ])
}
