use anchor_lang::prelude::*;

use crate::state::ConfigAccount;

pub fn config_init() {}

#[derive(Accounts)]
pub struct ConfigInit<'info> {
    #[account(mut)]
    pub config_authority: Signer<'info>,

    #[account(
        init,
        space = ConfigAccount::ACCOUNT_SIZE,
        payer = config_authority,
        seeds = [ConfigAccount::SEED_PREFIX.as_bytes()],
        bump
    )]
    pub config_account: Box<Account<'info, ConfigAccount>>,

    pub system_program: Program<'info, System>,
}
