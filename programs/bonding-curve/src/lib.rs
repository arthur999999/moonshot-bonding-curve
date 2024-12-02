use anchor_lang::prelude::*;

pub mod consts;
pub mod errors;
pub mod instructions;
pub mod state;

use crate::instructions::*;

declare_id!("HvUE85QPy1VHfPXVR1DrQy3nEddHvApHy9btbLvH1FnX");

#[program]
pub mod bonding_curve {

    use super::*;

    pub fn config_init(ctx: Context<ConfigInit>, params: ConfigParams) -> Result<()> {
        instructions::config_init(ctx, params)
    }
}
