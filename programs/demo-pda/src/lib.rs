use anchor_lang::prelude::*;

declare_id!("FVPoyuQPeYzydoJvLtaqVAvw8v4bCTXxEGV995x9TYC7"); 

#[program]
pub mod demo_pda {
    use super::*;

    // Declare our instruction
    pub fn post_review(ctx: Context<ReviewAccounts>, restaurant: String, review: String, rating: u8) -> Result<()> {
        
        // Create a new review
        // The review is the struct that we are saving on the PDA
        let new_review = &mut ctx.accounts.review;

        // The signer is the person who is creating the review and is paying for the transaction
        new_review.reviewer = ctx.accounts.signer.key();

        // Passing in the restaurant name, review, and rating
        new_review.restaurant = restaurant;
        new_review.review = review;
        new_review.rating = rating;

        // Print out the review
        // Normally, this would be saved to a database
        msg!("Restaurant review for {} - {} stars", new_review.restaurant, new_review.rating);
        msg!("Review: {}", new_review.review);

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(restaurant: String)]
pub struct ReviewAccounts<'info> {
    // This specifies the struct (class) that you want to save on the PDA
    // The payer is the person who is paying for the transaction, in this example it is the person created the review
    // The space is the amount of space that you want to allocate for the PDA
    #[account(
        init,
        seeds = [restaurant.as_bytes().as_ref(), signer.key().as_ref()],
        payer = signer,
        space = 500,
        bump
    )]
    pub review: Account<'info, Review>,

    // The signer is the person who is creating the review and is paying for the transaction
    #[account(mut)]
    pub signer: Signer<'info>,
    
    // The system program is required for all Solana programs
    pub system_program: Program<'info, System>,
}

// Declare all of the attributes that you want to save on the PDA
// in this case, we want to save the restaurant name, the review, and the rating
#[account]
pub struct Review {
    pub reviewer: Pubkey,
    pub restaurant: String,
    pub review: String,
    pub rating: u8
}
