use anchor_lang::prelude::*;

declare_id!("HEtLhVb4bn4SpueukorDMpyFfNHijyMi1vgFxErnjjPj");

mod state;
mod instructions;
mod error;

use instructions::*;

#[program]
pub mod amm {
    use super::*;
}
