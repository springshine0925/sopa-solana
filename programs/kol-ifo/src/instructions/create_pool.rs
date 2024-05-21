use crate::errors::KolIfoError;
use crate::seeds::{IFO_CONFIG_SEED, POOL_CONFIG_SEED};
use crate::state::{IFOConfig, PoolConfig};
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CreatePoolArgs {
    pub offering_amount: u64,
    pub raising_amount: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub claim_time: u64,
    pub min_amount: u64,
    pub max_amount: u64,
    pub over_funding: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct SupplementPoolArgs {
    pub pool_index: u128,
    pub token_mint: Option<Pubkey>,
    pub lp_token: Option<Pubkey>,
    pub initial_rate: u128,
    pub tn: u128,
    pub cliff: u128,
    pub period: u128,
    pub is_refund: bool,
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [IFO_CONFIG_SEED],
        bump = ifo_config.bump
    )]
    pub ifo_config: Box<Account<'info, IFOConfig>>,

    #[account(
        init,
        payer = signer,
        space = 1024,
        seeds = [POOL_CONFIG_SEED,ifo_config.pool_number.to_be_bytes().as_ref()],
        bump
    )]
    pub pool_config: Box<Account<'info, PoolConfig>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(args: SupplementPoolArgs)]
pub struct SupplementPool<'info> {
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
        seeds = [POOL_CONFIG_SEED,args.pool_index.to_be_bytes().as_ref()],
        bump=pool_config.bump
    )]
    pub pool_config: Box<Account<'info, PoolConfig>>,

    pub system_program: Program<'info, System>,
}

pub fn create_pool_handler(ctx: Context<CreatePool>, args: CreatePoolArgs) -> Result<()> {
    // only admin
    let ifo_config = &ctx.accounts.ifo_config;
    ifo_config.check_admin(ctx.accounts.signer.key())?;
    ctx.accounts.ifo_config.pool_number = ifo_config.pool_number + 1;

    require!(args.max_amount >= args.min_amount, KolIfoError::TimeError1);
    require!(args.end_time > args.start_time, KolIfoError::TimeError2);
    require!(args.claim_time >= args.end_time, KolIfoError::TimeError3);

    //set pool config
    let pool_config = &mut ctx.accounts.pool_config;
    pool_config.bump = ctx.bumps.pool_config;

    pool_config.start_time = args.start_time;
    pool_config.end_time = args.end_time;
    pool_config.claim_time = args.claim_time;
    pool_config.min_amount = args.min_amount;
    pool_config.max_amount = args.max_amount;
    pool_config.offering_amount = args.offering_amount;
    pool_config.raising_amount = args.raising_amount;
    pool_config.user_count = 0;
    pool_config.total_amount = 0;
    pool_config.over_funding = args.over_funding;

    Ok(())
}

pub fn supplement_pool_handler(
    ctx: Context<SupplementPool>,
    args: SupplementPoolArgs,
) -> Result<()> {
    // only admin
    let ifo_config = &ctx.accounts.ifo_config;
    ifo_config.check_admin(ctx.accounts.signer.key())?;

    //set pool config
    let pool_config = &mut ctx.accounts.pool_config;

    pool_config.offering_token_mint = args.token_mint;
    pool_config.is_refund = args.is_refund;

    require!(
        pool_config.raising_amount > 0,
        KolIfoError::RaisingAmountError
    );

    pool_config.tn = args.tn;
    pool_config.cliff = args.cliff;
    pool_config.period = args.period;
    pool_config.initial_rate = args.initial_rate;
    pool_config.lp_token_mint = args.lp_token;

    Ok(())
}
