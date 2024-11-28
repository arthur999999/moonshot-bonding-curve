use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

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
    pub fee_bps: Pubkey,
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
