use anchor_lang::prelude::*;

declare_id!("J4benTfF2csTh4GJcjrhC7zboKk8UyL8gc6B9RMGWD2H");

#[program]
pub mod anchor_crud_message {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     Ok(())
    // }
    pub fn creat(_ctx : Context<Create>) -> Result<()> {
        Ok(())
    }
    pub fn update(_ctx : Context<Update>) -> Result<()> {
        Ok(())
    }
    pub fn delete(_ctx : Context<Delete>) -> Result<()> {
        Ok(())
    }
    
}

// #[derive(Accounts)]
// pub struct Initialize {}
#[derive(Accounts)]
pub struct Create {

}
#[derive(Accounts)]
pub struct Update{

}
#[derive(Accounts)]
pub struct Delete {

}
#[account]
pub struct MessageAccount{
    pub user : Pubkey,
    pub message : String,
    pub bump : u8,
}