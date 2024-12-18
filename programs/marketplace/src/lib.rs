use anchor_lang::prelude::*;

declare_id!("ECyPjsZBXBAqqAMEeDPPJEkUHGfxMU54d9LpGm5MRAFy");

mod state;
mod instructions;
mod errors;

use instructions::*;

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
