use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, program::invoke_signed, pubkey::Pubkey},
};

use crate::IdentityLevel;

pub const POLICY_ENGINE_ID: &str = "4wyusm2nxncvNyiVej5mZ1k1mVPhfG41oNvHx4BWiYMq";

pub fn get_identity_registry_pda(asset_mint: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[asset_mint.as_ref()], &crate::ID).0
}

pub fn cpi_enforce_policy_on_levels_change<'info>(
    identity_account: AccountInfo<'info>,
    identity_registry: AccountInfo<'info>,
    asset_mint: AccountInfo<'info>,
    tracker_account: AccountInfo<'info>,
    policy_engine: AccountInfo<'info>,
    policy_program: AccountInfo<'info>,
    new_levels: &[IdentityLevel],
    new_country: u8,
    enforce_limits: bool,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let mut data: Vec<u8> = vec![236, 18, 123, 137, 253, 244, 32, 248];
    data.extend(new_levels.try_to_vec()?);
    data.extend(new_country.try_to_vec()?);
    data.extend((!enforce_limits).try_to_vec()?);
    invoke_signed(
        &Instruction {
            program_id: policy_program.key(),
            accounts: vec![
                // Mutable as authority is used as rent receiver on account closure.
                AccountMeta::new_readonly(identity_account.key(), false),
                AccountMeta::new_readonly(identity_registry.key(), true),
                AccountMeta::new_readonly(asset_mint.key(), false),
                AccountMeta::new_readonly(tracker_account.key(), false),
                AccountMeta::new(policy_engine.key(), false),
            ],
            data,
        },
        &[
            identity_account,
            identity_registry,
            asset_mint,
            tracker_account,
            policy_engine,
        ],
        signer_seeds,
    )?;

    Ok(())
}
