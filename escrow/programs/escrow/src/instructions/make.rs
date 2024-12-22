use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{Escrow, ESCROW_SEED};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
      mint::token_program = token_program
  )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
      mint::token_program = token_program
  )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
      mut,
      associated_token::mint = mint_a,
      associated_token::authority = maker,
  )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
      init,
      payer = maker,
      space = 8 + Escrow::INIT_SPACE,
      seeds = [
        ESCROW_SEED.as_bytes(),
        maker.key().as_ref(),
        seed.to_le_bytes().as_ref(),
        mint_a.key().as_ref(),
        mint_b.key().as_ref()
      ],
      bump
  )]
    pub escrow: Account<'info, Escrow>,

    #[account(
      init,
      payer = maker,
      associated_token::mint = mint_a,
      associated_token::authority = escrow,
  )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    pub fn init_escrow_and_deposit(
        &mut self,
        seed: u64,
        receive_amount: u64,
        deposit: u64,
        bumps: &MakeBumps,
    ) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive_amount,
            bump: bumps.escrow,
        });

        msg!("Escrow initialized");

        let transfer_accounts = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            to: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer_checked(cpi_ctx, deposit, self.mint_a.decimals)
    }
}
