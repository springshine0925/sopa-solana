use anchor_lang::prelude::*;

mod instructions;
mod state;
mod seeds;
mod errors;

use instructions::*;

declare_id!("DpbL8PgWgFMnSrAXYBst7Fuhx7ne14JsVisYRdVCDnkY");

#[program]
pub mod kol_ifo {
    use super::*;
    pub fn initialize(ctx: Context<Init>, args: InitArgs) -> Result<()> {
        init_handler(ctx, args)
    }

    pub fn create_pool<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CreatePool<'info>>,
        args: CreatePoolArgs,
    ) -> Result<()> {
        create_pool_handler(ctx, args)
    }

    pub fn supplement_pool<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, SupplementPool<'info>>,
        args: SupplementPoolArgs,
    ) -> Result<()> {
        supplement_pool_handler(ctx, args)
    }

    pub fn claim<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, UpdateUser<'info>>,
        args: UpdateUserArgs,
    ) -> Result<()> {
        claim_pool_handler(ctx, args)
    }



    pub fn update_ifo<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, UpdateIFO<'info>>,
        args: UpdateIFOArgs,
    ) -> Result<()> {
        update_ifo_handler(ctx, args)
    }

    pub fn withdraw<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Withdraw<'info>>,
        args: WithdrawArgs,
    ) -> Result<()> {
        withdraw_handler(ctx, args)
    }

}
