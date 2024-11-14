use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub seed: u64, // used for creating different pools config
    pub authority: Option<Pubkey>, // if we want authority to lock config account
    pub mint_x : Pubkey, // TokenX
    pub mint_y: Pubkey, // TokenY
    pub fee: u16, // Swap fee in basis points
    pub locked: bool, // authority can lock a pool
    pub config_bump: u8, // seeds for config account
    pub lp_bump: u8 // bump seeds for LP token
}