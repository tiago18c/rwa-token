use crate::{state::*, PolicyEngineErrors};
use anchor_lang::{
    prelude::*,
    solana_program::sysvar::{self, instructions::get_instruction_relative},
};
use anchor_spl::token_2022;
use identity_registry::IdentityLevel;
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};

/// enforce receiver
/// enforce sender
/// 
/// enforce both
/// 
pub fn enforce_identity_filter2(identity: &[IdentityLevel], identity_filter: IdentityFilter, timestamp: i64) -> Result<()> {
    match identity_filter.comparision_type {
        ComparisionType::Or => {
            // if any level is in the identities array, return Ok
            for level in identity.iter() {
                if level.expiry > timestamp && identity_filter.identity_levels.contains(&level.level) {
                    return Ok(());
                }
            }
            Err(PolicyEngineErrors::IdentityFilterFailed.into())
        }
        ComparisionType::And => {
            // if all levels are in the identities array, return Ok
            for level in identity.iter() {
                if level.expiry > timestamp && !identity_filter.identity_levels.contains(&level.level) {
                    return Err(PolicyEngineErrors::IdentityFilterFailed.into());
                }
            }
            Ok(())
        }
        ComparisionType::Except => {
            // if any level is in the identities array, return Err
            for level in identity.iter() {
                if level.expiry > timestamp && identity_filter.identity_levels.contains(&level.level) {
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
            enforce_identity_filter2(destination_identity, identity_filter, timestamp)
            .or(enforce_identity_filter2(source_identity, identity_filter, timestamp))
        }
        (_, CounterpartyFilter::Receiver) => {
            enforce_identity_filter2(destination_identity, identity_filter, timestamp)
        }
        (_, CounterpartyFilter::Sender) => {
            enforce_identity_filter2(source_identity, identity_filter, timestamp)
        }
        (_, CounterpartyFilter::Both) => 
        {
            enforce_identity_filter2(destination_identity, identity_filter, timestamp)?;
            enforce_identity_filter2(source_identity, identity_filter, timestamp)
        }
    }
}

pub fn enforce_identity_filter(destination_identity: &[IdentityLevel], source_identity: &[IdentityLevel], identity_filter: IdentityFilter, timestamp: i64) -> Result<()> {
    match identity_filter.comparision_type {
        ComparisionType::Or => {
            // if any level is in the identities array, return Ok
            if identity_filter.counterparty_filter == CounterpartyFilter::Receiver || identity_filter.counterparty_filter == CounterpartyFilter::Both {
                for level in destination_identity.iter() {
                    if level.expiry > timestamp && identity_filter.identity_levels.contains(&level.level) {
                        return Ok(());
                    }
                }
            }
            if identity_filter.counterparty_filter == CounterpartyFilter::Sender || identity_filter.counterparty_filter == CounterpartyFilter::Both {
                for level in source_identity.iter() {
                    if level.expiry > timestamp && identity_filter.identity_levels.contains(&level.level) {
                        return Ok(());
                    }
                }
            }
            Err(PolicyEngineErrors::IdentityFilterFailed.into())
        }
        ComparisionType::And => {
            // if all levels are in the identities array, return Ok
            if identity_filter.counterparty_filter == CounterpartyFilter::Receiver || identity_filter.counterparty_filter == CounterpartyFilter::Both {
                for level in destination_identity.iter() {
                    if level.expiry > timestamp && !identity_filter.identity_levels.contains(&level.level) {
                        return Err(PolicyEngineErrors::IdentityFilterFailed.into());
                    }
                }
            }
            if identity_filter.counterparty_filter == CounterpartyFilter::Sender || identity_filter.counterparty_filter == CounterpartyFilter::Both {
                for level in source_identity.iter() {
                    if level.expiry > timestamp && !identity_filter.identity_levels.contains(&level.level) {
                        return Err(PolicyEngineErrors::IdentityFilterFailed.into());
                    }
                }
            }
            Ok(())
        }
        ComparisionType::Except => {
            // if any level is in the identities array, return Err
            if identity_filter.counterparty_filter == CounterpartyFilter::Receiver || identity_filter.counterparty_filter == CounterpartyFilter::Both {
                for level in destination_identity.iter() {
                    if level.expiry > timestamp && identity_filter.identity_levels.contains(&level.level) {
                        return Err(PolicyEngineErrors::IdentityFilterFailed.into());
                    }
                }
            }
            if identity_filter.counterparty_filter == CounterpartyFilter::Sender || identity_filter.counterparty_filter == CounterpartyFilter::Both {
                for level in source_identity.iter() {
                    if level.expiry > timestamp && identity_filter.identity_levels.contains(&level.level) {
                        return Err(PolicyEngineErrors::IdentityFilterFailed.into());
                    }
                }
            }
            Ok(())
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

pub const TRANSFER_HOOK_MINT_INDEX: usize = 1;

pub fn verify_cpi_program_is_token22(
    instructions_program: &AccountInfo,
    amount: u64,
    mint: Pubkey,
) -> Result<()> {
    let ix_relative = get_instruction_relative(0, instructions_program)?;
    if ix_relative.program_id != token_2022::ID {
        return Err(PolicyEngineErrors::InvalidCpiTransferProgram.into());
    }
    if ix_relative.data[1..9] != amount.to_le_bytes() {
        return Err(PolicyEngineErrors::InvalidCpiTransferAmount.into());
    }
    // make sure transfer mint is same
    if let Some(account) = ix_relative.accounts.get(TRANSFER_HOOK_MINT_INDEX) {
        if account.pubkey != mint {
            return Err(PolicyEngineErrors::InvalidCpiTransferMint.into());
        }
    } else {
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
        // instructions program
        ExtraAccountMeta::new_with_pubkey(&sysvar::instructions::id(), false, false)?,
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
