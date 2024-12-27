use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(removed_counters: Vec<u8>, added_counters: Vec<Counter>)]
pub struct ChangeCounters<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        constraint = policy_engine.authority == signer.key() || policy_engine.delegate == signer.key()
    )]
    pub signer: Signer<'info>,
    #[account(mut,
        realloc = policy_engine.to_account_info().data_len() + Counter::INIT_SPACE * added_counters.len() - Counter::INIT_SPACE * removed_counters.len(),
        realloc::zero = false,
        realloc::payer = payer,
    )]
    pub policy_engine: Box<Account<'info, PolicyEngineAccount>>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ChangeCounters>,
    removed_counters: Vec<u8>,
    added_counters: Vec<Counter>,
) -> Result<()> {
    let size_diff = ctx.accounts
        .policy_engine
        .update_counters(removed_counters, added_counters)?;


    let space = if size_diff > 0 {
        ctx.accounts.policy_engine.to_account_info().data_len() + size_diff as usize

    } else {
        ctx.accounts.policy_engine.to_account_info().data_len() - (-1*size_diff) as usize
    };

    let rent = Rent::get()?.minimum_balance(space);

    if rent > ctx.accounts.policy_engine.to_account_info().lamports() {
        anchor_lang::system_program::transfer(
            anchor_lang::context::CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.policy_engine.to_account_info(),
                },
            ),
            rent.checked_sub(ctx.accounts.policy_engine.to_account_info().lamports()).unwrap(),
        )?;
    } else if rent < ctx.accounts.policy_engine.to_account_info().lamports() {
        let diff = ctx.accounts.policy_engine.to_account_info().lamports().checked_sub(rent).unwrap();
        **ctx.accounts.payer.to_account_info().lamports.borrow_mut() = ctx.accounts.payer.to_account_info().lamports().checked_add(diff).unwrap();
        **ctx.accounts.policy_engine.to_account_info().lamports.borrow_mut() = ctx.accounts.policy_engine.to_account_info().lamports().checked_sub(diff).unwrap();
    }

    ctx.accounts.policy_engine.to_account_info().realloc(space, false)?;
    Ok(())
}
