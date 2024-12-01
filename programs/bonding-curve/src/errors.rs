use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("Config field needs to be present during initialization")]
    ConfigFiledMissing,
}
