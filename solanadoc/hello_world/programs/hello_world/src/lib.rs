use anchor_lang::prelude::*;

declare_id!("Cgex5vBqXkhjEhNAoRjEYuwEGwaG4tvAk1DMVsJSFRou");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
