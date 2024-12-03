use anchor_lang::prelude::*;

use crate::state::ConfigAccount;

use super::ConfigParams;

pub fn config_update(ctx: Context<ConfigUpdate>, config_params: ConfigParams) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct ConfigUpdate<'info> {
    #[account(mut)]
    pub config_authority: Signer<'info>,

    #[account(mut)]
    pub config_account: Box<Account<'info, ConfigAccount>>,

    pub system_program: Program<'info, System>,
}
