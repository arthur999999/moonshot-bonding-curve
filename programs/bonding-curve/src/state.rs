use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

use crate::{errors::Errors, instructions::config_init::ConfigParams};

#[account]
pub struct CurveAccount {
    pub total_supply: u64,
    pub curve_amount: u64,
    pub mint: Pubkey,
    pub decimals: u8,
    pub collateral_currency: Currency,
    pub curve_type: CurveType,
    pub marketcap_threshold: u64,
    pub marketcap_currency: Currency,
    pub migration_fee: u64,
    pub coef_b: u32,
    pub bump: u8,
    pub migration_target: MigrationTarget,
}

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub enum CurveType {
    LinearV1,
    ConstantProduct,
}

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub enum Currency {
    Sol,
}

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub enum MigrationTarget {
    Raydium,
    Meteora,
}

#[account]
pub struct ConfigAccount {
    pub migration_authority: Pubkey,
    pub backend_authority: Pubkey,
    pub config_authority: Pubkey,
    pub helio_fee: Pubkey,
    pub dex_fee: Pubkey,
    pub fee_bps: u16,
    pub dex_fee_share: u8,
    pub migration_fee: u64,
    pub marketcap_threshold: u64,
    pub marketcap_currency: Currency,
    pub min_supported_decimal_places: u8,
    pub max_supported_decimal_places: u8,
    pub min_supported_token_supply: u64,
    pub max_supported_token_supply: u64,
    pub bump: u8,
    pub coef_b: u32,
}

impl ConfigAccount {
    pub const ACCOUNT_SIZE: usize =
        8 +   // discriminator 
        32 + 
        32 + 
        32 + 
        32 + 
        32 + 
        4 + 
        1 + 
        8 + 
        8 + 
        1 + 
        1 + 
        1 + 
        8 + 
        8 + 
        1 + 
        4;

    pub const SEED_PREFIX: &str = "config_account";

    pub fn new(config_params: ConfigParams, bump: u8) -> Result<Self> {
        Ok(Self{ 
            migration_authority: config_params.migration_authority.ok_or(Errors::ConfigFiledMissing)?, 
            backend_authority: config_params.backend_authority.ok_or(Errors::ConfigFiledMissing)?,
            config_authority: config_params.config_authority.ok_or(Errors::ConfigFiledMissing)?,
            helio_fee: config_params.helio_fee.ok_or(Errors::ConfigFiledMissing)?,
            dex_fee: config_params.dex_fee.ok_or(Errors::ConfigFiledMissing)?,
            fee_bps: config_params.fee_bps.ok_or(Errors::ConfigFiledMissing)?,
            dex_fee_share: config_params.dex_fee_share.ok_or(Errors::ConfigFiledMissing)?,
            migration_fee: config_params.migration_fee.ok_or(Errors::ConfigFiledMissing)?,
            marketcap_threshold: config_params.marketcap_threshold.ok_or(Errors::ConfigFiledMissing)?,
            marketcap_currency: Currency::Sol,
            min_supported_decimal_places: config_params.min_supported_decimal_places.ok_or(Errors::ConfigFiledMissing)?,
            max_supported_decimal_places: config_params.max_supported_decimal_places.ok_or(Errors::ConfigFiledMissing)?,
            min_supported_token_supply: config_params.min_supported_token_supply.ok_or(Errors::ConfigFiledMissing)?,
            max_supported_token_supply: config_params.max_supported_token_supply.ok_or(Errors::ConfigFiledMissing)?, 
            bump, 
            coef_b: config_params.coef_b.ok_or(Errors::ConfigFiledMissing)?
        })
    }
}
