use anchor_lang::prelude::*;

declare_id!("ECyPjsZBXBAqqAMEeDPPJEkUHGfxMU54d9LpGm5MRAFy");

mod errors;
mod instructions;
mod state;

pub use instructions::*;
pub use state::*;

#[program]
pub mod marketplace {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u64) -> Result<()> {
        ctx.accounts.initialize(name, fee, &ctx.bumps)
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()>{
        ctx.accounts.send_sol()?;
        ctx.accounts.send_nft()?;
        ctx.accounts.close_mint_vault()
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()>{
        ctx.accounts.withdraw_nft()?;
        ctx.accounts.close_mint_vault()
    }
}
