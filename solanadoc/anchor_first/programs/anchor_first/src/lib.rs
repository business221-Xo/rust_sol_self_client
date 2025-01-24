// use anchor_lang::prelude::*;

// declare_id!("ETNsx7b2F2qDBDRrtNEFbfZBZBkZjvwWn19jN17533pT");

// #[program]
// pub mod anchor_first {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}


use anchor_lang::prelude::*;
declare_id!("ETNsx7b2F2qDBDRrtNEFbfZBZBkZjvwWn19jN17533pT");

#[program]
pub mod anchor_first {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data : u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Hello, Anchor, changed data to {}", data );
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8 )]
    pub new_account : Account<'info, NewAccount>,
    #[account(mut)]
    pub signer : Signer<'info>,
    pub system_program : Program<'info, System>,
}
#[account]
pub struct NewAccount{
    data : u64
}