use anchor_lang::prelude::*;

use crate::{consts::CONFIG_INIT_AUTHORITY, errors::Errors, state::ConfigAccount};

pub fn config_init(ctx: Context<ConfigInit>, config_params: ConfigParams) -> Result<()> {
    //only authorized accounts can create account configs
    if ctx.accounts.config_authority.key.to_string() != CONFIG_INIT_AUTHORITY {
        return err!(Errors::InvalidAuthority);
    }

    let config = &mut ctx.accounts.config_account;

    let config_account = ConfigAccount::new(config_params, ctx.bumps.config_account)?;

    config.set_inner(config_account);

    Ok(())
}

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

#[derive(Debug, AnchorSerialize, AnchorDeserialize)]
pub struct ConfigParams {
    pub migration_authority: Option<Pubkey>,
    pub backend_authority: Option<Pubkey>,
    pub config_authority: Option<Pubkey>,
    pub helio_fee: Option<Pubkey>,
    pub dex_fee: Option<Pubkey>,
    pub fee_bps: Option<u16>,
    pub dex_fee_share: Option<u8>,
    pub migration_fee: Option<u64>,
    pub marketcap_threshold: Option<u64>,
    pub marketcap_currency: Option<u8>,
    pub min_supported_decimal_places: Option<u8>,
    pub max_supported_decimal_places: Option<u8>,
    pub min_supported_token_supply: Option<u64>,
    pub max_supported_token_supply: Option<u64>,
    pub coef_b: Option<u32>,
}
