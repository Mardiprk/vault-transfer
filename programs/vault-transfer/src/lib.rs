use anchor_lang::prelude::*;

declare_id!("94vd5bjzHPXHnN6w6s7gNrmrr5JwF1jSgVjzyZNRPTYs");

#[program]
pub mod vault_transfer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
