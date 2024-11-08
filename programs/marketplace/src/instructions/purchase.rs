use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked};
use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = taker
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close = maker,
        seeds = [b"listing", marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
     seeds = [b"treasury", marketplace.key().as_ref()],
      bump = marketplace.treasury_bump
    )]
    pub treasury: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump = marketplace.rewards_bump,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}


impl<'info> Purchase<'info> {
    pub fn send_sol(&mut self) -> Result<()> {
        // Transfer purchase amount from taker to maker Solana account
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer { from: self.maker.to_account_info(), to: self.taker.to_account_info() };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        let amount = self.listing.price.checked_sub(self.marketplace.fee as u64).unwrap();
        transfer(cpi_context, amount)?;
        Ok(())
    }

    pub fn send_nft(&mut self) -> Result<()> {
        // Transfer NFT from vault to taker ATA
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked { from: self.vault.to_account_info(), to: self.taker_ata.to_account_info(), mint: self.maker_mint.to_account_info(), authority: self.listing.to_account_info() };
        let seeds = &[
            &"listing".as_bytes(),
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_context, 1, 0)?;
        Ok(())
    }

    //     send money to treasury
    //      Mint rewards
    //     close the vault
}