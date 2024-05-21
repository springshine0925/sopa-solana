use crate::state::IFOConfig;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InitArgs {
    pub admin: Pubkey,
    pub manager: [u8; 64],
}

use crate::seeds::IFO_CONFIG_SEED;

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 256,
        seeds = [IFO_CONFIG_SEED],
        bump
    )]
    pub contracts_config: Box<Account<'info, IFOConfig>>,
    pub system_program: Program<'info, System>,
}

pub fn init_handler(ctx: Context<Init>, args: InitArgs) -> Result<()> {
    let contracts_config = &mut ctx.accounts.contracts_config;
    contracts_config.bump = ctx.bumps.contracts_config;
    contracts_config.admin = args.admin;
    contracts_config.manager = args.manager;
    Ok(())
}
