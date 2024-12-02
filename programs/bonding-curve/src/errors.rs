use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("Invalid Authority provided.")]
    InvalidAuthority,
    #[msg("Config field needs to be present during initialization")]
    ConfigFiledMissing,
}
