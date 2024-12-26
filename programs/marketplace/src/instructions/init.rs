use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenInterface, Mint};
use crate::errors::MarketplaceError;
use crate::state::Marketplace;

#[derive(Accounts)]
#[instruction(name:String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace", name.as_str().as_bytes()],
        space = 8 + Marketplace::INIT_SPACE,
        bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = marketplace,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,
    
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, name: String, fee: u64, bumps: &InitializeBumps) -> Result<()> {
        // Check String length
        require!(name.len() > 0 && name.len() <  32, MarketplaceError::StringLengthInvalid);

        // Initialize marketplace
        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee,
            name,
            treasury_bump: bumps.treasury,
            rewards_bump: bumps.rewards_mint,
            bump: bumps.marketplace,
        });
        Ok(())
    }
}