use anchor_lang::prelude::*;

use solend_anchor::init_lending_market;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod example {

    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        
        // let cpi_context = 

        // init_lending_market(ctx, quote_currency);
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
