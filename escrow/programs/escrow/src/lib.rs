pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("GmqgzDmzU9b35bg6CBqm1wcXwcp7tVCCpo78ptymVjes");

#[program]
pub mod escrow_program {
    use super::*;

    pub fn init_escrow_and_deposit(
        ctx: Context<Make>,
        seed: u64,
        receive_amount: u64,
        deposit: u64,
    ) -> Result<()> {
        ctx.accounts
            .init_escrow_and_deposit(seed, receive_amount, deposit, &ctx.bumps)
    }

    pub fn cancel_and_refund(ctx: Context<Cancel>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    pub fn take_and_close_vault(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.take_and_close_vault()
    }
}