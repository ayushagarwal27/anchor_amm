use anchor_lang::prelude::*;

declare_id!("HEtLhVb4bn4SpueukorDMpyFfNHijyMi1vgFxErnjjPj");

mod error;
mod instructions;
mod state;

use instructions::*;

#[program]
pub mod amm {
    use deposit::Deposit;

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts
            .initialize_config(seed, fee, authority, &ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)
    }

    pub fn swap(ctx: Context<Swap>, is_x: bool, amount: u64, min: u64) -> Result<()> {
        ctx.accounts.swap(is_x, amount, min)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64, min_x: u64, min_y: u64) -> Result<()> {
        ctx.accounts.withdraw(amount, min_x, min_y)
    }
}
