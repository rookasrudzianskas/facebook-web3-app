use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// Post and comment text length
const TEXT_LENGTH: usize = 1024;
// Username length
const USER_LENGTH: usize = 100;
// URL for image
const USER_URL_LENGTH: usize = 255;

#[program]
pub mod programs {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

// Contexts
// CreateState context
#[derive(Accounts)]
pub struct CreateState<'info> {
    // Authenticating state account
    #[account(
    init,
    seeds = [b"state".as_ref()],
    bump,
    payer = authority,
    space = size_of::<StateAccount>() + 8
    )]
    pub state: Account<'info, StateAccount>,

    // Authority (this is signer who paid transaction fee)
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System program
    pub system_program: UncheckedAccount<'info>,

    // Token program
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,
}

// CreatePost context
#[derive(Accounts)]
pub struct CreatePost<'info> {
    // Authenticate state account
    #[account(mut, seeds = [b"state".as_ref()], bump)]
    pub state: Account<'info, StateAccount>,

    // Authenticate post account
    // Post account use string "post" and index of post as seeds
    #[account(
    init,
    seeds = [b"post".as_ref(), state.post_count.to_be_bytes().as_ref()],
    bump,
    payer = authority,
    space = size_of::<PostAccount>() + TEXT_LENGTH + USER_NAME_LENGTH + USER_URL_LENGTH
    )]
    pub post: Account<'info, PostAccount>,

    // Authority (this is signer who paid transaction fee)
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System program
    pub system_program: UncheckedAccount<'info>,

    // Token program
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,

    // Clock to save time
    pub clock: Sysvar<'info, Clock>,
}


// State Account Structure
#[account]
pub struct StateAccount {
    // Signer address
    pub authority: Pubkey,

    // Post count
    pub post_count: u64,
}


// Post Account Structure
#[account]
pub struct PostAccount {
    // Signer address
    pub authority: Pubkey,

    // Post text
    pub text: String,

    // Post creator name
    pub poster_name: String,

    // Post creator url
    pub poster_url: String,

    // Comment counts of post
    pub comment_count: u64,

    // Post index
    pub index: u64,

    // Post time
    pub post_time: i64,
}


