use anchor_lang::prelude::*;

pub mod errors;
mod instructions;
pub mod seeds;
pub mod state;

use instructions::*;

declare_id!("GwLvNYDs95cP4FNCLcvLNG7wurP5oQk8KdfyA5MGnsk5");

#[program]
pub mod sopad_solana {
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

    pub fn deposit_pool<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, DepositPool<'info>>,
        args: DepositPoolArgs,
    ) -> Result<()> {
        deposit_pool_handler(ctx, args)
    }

    pub fn deposit_sol_pool<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, DepositSolPool<'info>>,
        args: DepositSolPoolArgs,
    ) -> Result<()> {
        deposit_sol_pool_handler(ctx, args)
    }

    pub fn claim<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, UpdateUser<'info>>,
        args: UpdateUserArgs,
    ) -> Result<()> {
        claim_pool_handler(ctx, args)
    }

    pub fn update_pool<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, UpdatePool<'info>>,
        args: UpdatePoolArgs,
    ) -> Result<()> {
        update_pool_handler(ctx, args)
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

    pub fn get_refunding_amount_fun<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, GetClaimAmount<'info>>,
        args: GetClaimAmountArgs,
    ) -> Result<()> {
        // solana_program::program::set_return_data(b"Custom return value");
        match get_refunding_amount_handler(ctx, args) {
            Ok(v) => {
                solana_program::program::set_return_data(&v.to_be_bytes());
            }
            Err(_e) => {
                solana_program::program::set_return_data(&0_i32.to_be_bytes());
            }
        }
        Ok(())
    }

    //用户可领取
    pub fn get_offering_amount_fun<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, GetClaimAmount<'info>>,
        args: GetClaimAmountArgs,
    ) -> Result<()> {
        // solana_program::program::set_return_data(b"Custom return value");
        match get_offering_amount_handler(ctx, args) {
            Ok(v) => {
                solana_program::program::set_return_data(&v.to_be_bytes());
            }
            Err(_e) => {
                solana_program::program::set_return_data(&0_i32.to_be_bytes());
            }
        }
        Ok(())
    }

    pub fn get_claim_amount_fun<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, GetClaimAmount<'info>>,
        _args: GetClaimAmountArgs,
    ) -> Result<()> {
        // solana_program::program::set_return_data(b"Custom return value");
        match get_claim_amount_handler(ctx) {
            Ok(v) => {
                solana_program::program::set_return_data(&v.to_be_bytes());
            }
            Err(_e) => {
                solana_program::program::set_return_data(&0_i32.to_be_bytes());
            }
        }
        Ok(())
    }

    pub fn refund<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Refund<'info>>,
        args: RefundArgs,
    ) -> Result<()> {
        refund_handler(ctx, args)
    }
}
