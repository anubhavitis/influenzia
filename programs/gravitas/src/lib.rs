use anchor_lang::prelude::*;

declare_id!("7nkqBwkCschzVvRymkppmcfjVzkEn1VnseemVprVofjX");

#[program]
pub mod gravitas {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}