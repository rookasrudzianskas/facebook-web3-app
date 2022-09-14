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

    pub fn create_state(ctx: Context<CreateState>) -> Result<()> {
        // Get state from context
        let state = &mut ctx.accounts.state;
        // Save authority to state
        state.authority = ctx.accounts.authority.key();
        // Set post count as 0 when initializing
        state.post_count = 0;
        Ok(())
    }

    pub fn create_post(
        ctx: Context<CreatePost>,
        text: String,
        poster_name: String,
        poster_url: String,
    ) -> Result<()> {
        // Get State
        let state = &mut ctx.accounts.state;

        // Get post
        let post = &mut ctx.accounts.post;
        // Set authority
        post.authority = ctx.accounts.authority.key();
        // Set text
        post.text = text;
        // Set poster name
        post.poster_name = poster_name;
        // Set poster avatar url
        post.poster_url = poster_url;
        // Set comment count as 0
        post.comment_count = 0;
        // Set post index as state's post count
        post.index = state.post_count;
        // Set post time
        post.post_time = ctx.accounts.clock.unix_timestamp;
        // Increase state's post count by 1
        state.post_count += 1;
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

    // System program
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


