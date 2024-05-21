use crate::errors::SoPadError;
use crate::state::{IFOConfig, PoolConfig};
use anchor_lang::prelude::*;

use crate::seeds::{IFO_CONFIG_SEED, POOL_CONFIG_SEED};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct UpdatePoolArgs {
    pub pool_idex: u128,
    pub offering_amount: Option<u64>,
    pub raising_amount: Option<u64>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub claim_time: Option<u64>,
    pub min_amount: Option<u64>,
    pub max_amount: Option<u64>,
    pub initial_rate: Option<u128>,
    pub tn: Option<u128>,
    pub cliff: Option<u128>,
    pub period: Option<u128>,
}

#[derive(Accounts)]
#[instruction(args: UpdatePoolArgs)]
pub struct UpdatePool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [IFO_CONFIG_SEED],
        bump = ifo_config.bump
    )]
    pub ifo_config: Box<Account<'info, IFOConfig>>,

    #[account(
        mut,
        seeds = [POOL_CONFIG_SEED,args.pool_idex.to_be_bytes().as_ref()],
        bump=pool_config.bump
    )]
    pub pool_config: Box<Account<'info, PoolConfig>>,

    pub system_program: Program<'info, System>,
}

pub fn update_pool_handler(ctx: Context<UpdatePool>, args: UpdatePoolArgs) -> Result<()> {
    // only admin
    let ifo_config = &ctx.accounts.ifo_config;
    ifo_config.check_admin(ctx.accounts.signer.key())?;

    let clock = Clock::get().unwrap();
    require!(
        (clock.unix_timestamp as u64) < ctx.accounts.pool_config.start_time,
        SoPadError::PoolHasStarted
    );

    let pool_config = &mut ctx.accounts.pool_config;

    if let Some(offering_amount) = args.offering_amount {
        pool_config.offering_amount = offering_amount;
        msg!("offering_amount update");
    }

    if let Some(raising_amount) = args.raising_amount {
        pool_config.raising_amount = raising_amount;
        msg!("raising_amount update");
    }

    if let Some(claim_time) = args.claim_time {
        require!(claim_time >= pool_config.end_time, SoPadError::TimeError3);
        pool_config.claim_time = claim_time;
        msg!("claim_time update");
    }

    if let Some(end_time) = args.end_time {
        require!(end_time > pool_config.start_time, SoPadError::TimeError2);
        require!(pool_config.claim_time >= end_time, SoPadError::TimeError3);
        pool_config.end_time = end_time;
        msg!("end_time update");
    }

    if let Some(start_time) = args.start_time {
        require!(pool_config.end_time > start_time, SoPadError::TimeError2);
        pool_config.start_time = start_time;
        msg!("start_time update");
    }

    if let Some(min_amount) = args.min_amount {
        require!(pool_config.max_amount >= min_amount, SoPadError::ParamError);
        pool_config.min_amount = min_amount;
        msg!("min_amount update");
    }

    if let Some(max_amount) = args.max_amount {
        require!(max_amount >= pool_config.min_amount, SoPadError::ParamError);
        pool_config.max_amount = max_amount;
        msg!("max_amount update");
    }

    if let Some(initial_rate) = args.initial_rate {
        require!(pool_config.raising_amount > 0, SoPadError::ParamError);
        pool_config.initial_rate = initial_rate;
        msg!("initial_rate update");
    }

    if let Some(tn) = args.tn {
        require!(pool_config.raising_amount > 0, SoPadError::ParamError);
        pool_config.tn = tn;
        msg!("tn update");
    }

    if let Some(cliff) = args.cliff {
        require!(pool_config.cliff > 0, SoPadError::ParamError);
        pool_config.cliff = cliff;
        msg!("cliff update");
    }

    if let Some(period) = args.period {
        require!(pool_config.raising_amount > 0, SoPadError::ParamError);
        pool_config.period = period;
        msg!("period update");
    }

    Ok(())
}
