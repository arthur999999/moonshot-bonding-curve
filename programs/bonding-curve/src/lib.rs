use anchor_lang::prelude::*;

pub mod state;

declare_id!("HvUE85QPy1VHfPXVR1DrQy3nEddHvApHy9btbLvH1FnX");

#[program]
pub mod bonding_curve {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=sender_of_tweet, space= 89)]
    my_test: Account<'info, TweetOnSolana>,
    #[account(mut)]
    pub sender_of_tweet: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TweetOnSolana {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
}
