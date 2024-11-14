use anchor_lang::prelude::*;

declare_id!("HEtLhVb4bn4SpueukorDMpyFfNHijyMi1vgFxErnjjPj");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}