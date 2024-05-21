use crate::state::IFOConfig;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct UpdateIFOArgs {
    pub admin: Option<Pubkey>,
}

use crate::seeds::IFO_CONFIG_SEED;

#[derive(Accounts)]
pub struct UpdateIFO<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [IFO_CONFIG_SEED],
        bump=ifo_config.bump
    )]
    pub ifo_config: Box<Account<'info, IFOConfig>>,
    pub system_program: Program<'info, System>,
}

pub fn update_ifo_handler(ctx: Context<UpdateIFO>, args: UpdateIFOArgs) -> Result<()> {
    let ifo_config: &mut Box<Account<IFOConfig>> = &mut ctx.accounts.ifo_config;
    ifo_config.check_admin(ctx.accounts.signer.key())?;

    if let Some(admin) = args.admin {
        ifo_config.admin = admin;
        msg!("admin update");
    }

    Ok(())
}
