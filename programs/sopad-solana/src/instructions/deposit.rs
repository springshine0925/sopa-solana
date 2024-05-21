use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};
// use solana_sdk::signature::Signature;

use crate::{
    errors::SoPadError,
    get_offering_amount, get_refunding_amount,
    seeds::*,
    state::{only_signer, DepositOrder, IFOConfig, PoolConfig, UserConfig},
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DepositPoolArgs {
    pub amount: u64,
    pub pool_index: u128,
    pub max_amount: u64,
    pub signature: [u8; 64],
    pub message: Vec<u8>,
    pub recovery_id: u8,
}

#[derive(Accounts)]
#[instruction(args: DepositPoolArgs)]
pub struct DepositPool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [IFO_CONFIG_SEED],
        bump=ifo_config.bump
    )]
    pub ifo_config: Box<Account<'info, IFOConfig>>,

    #[account(
        mut,
        seeds = [POOL_CONFIG_SEED,args.pool_index.to_be_bytes().as_ref()],
        bump=pool_config.bump,
    )]
    pub pool_config: Box<Account<'info, PoolConfig>>,

    /// CHECK: This account is not read or written
    #[account()]
    pub lp_token_mint: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8+UserConfig::INIT_SPACE,
        seeds = [USER_CONFIG_SEED,signer.key().as_ref()],
        bump
    )]
    pub user_config: Box<Account<'info, UserConfig>>,

    #[account(
        mut,
        associated_token::mint = lp_token_mint,
        associated_token::authority = signer,
    )]
    pub user_lp_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = lp_token_mint,
        associated_token::authority = pool_config
    )]
    pub pool_lp_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer=signer,
        space=8+DepositOrder::INIT_SPACE,
        seeds = [
            DEPOSIT_ORDER_SEED,
            pool_config.key().as_ref(),
            pool_config.total_deposit_amount.to_be_bytes().as_ref()
        ],
        bump,
    )]
    pub deposit_order_account: Box<Account<'info, DepositOrder>>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> DepositPool<'info> {
    pub fn deposit_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_lp_token_account.to_account_info(),
            to: self.pool_lp_token_account.to_account_info(),
            authority: self.signer.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn deposit_pool_handler(ctx: Context<DepositPool>, args: DepositPoolArgs) -> Result<()> {
    // only signer
    let ifo_config = &ctx.accounts.ifo_config;
    only_signer(
        args.signature,
        args.message,
        args.recovery_id,
        ifo_config.manager.clone(),
    )?;

    let clock = Clock::get().unwrap();
    let time = clock.unix_timestamp as u64;
    //check params
    let pool_config = &ctx.accounts.pool_config;
    require!(time > pool_config.start_time, SoPadError::ParamError);
    require!(time < pool_config.end_time, SoPadError::ParamError);
    require!(
        args.amount >= pool_config.min_amount,
        SoPadError::ParamError
    );
    require!(args.amount > 0, SoPadError::ParamError);

    if args.max_amount > 0 {
        let user_config = &ctx.accounts.user_config;
        require!(
            user_config.amount + args.amount <= args.max_amount,
            SoPadError::ParamError
        );
    }

    if !pool_config.over_funding {
        require!(
            pool_config.total_amount <= pool_config.raising_amount,
            SoPadError::ParamError
        );
    }

    // transfer amount
    msg!("tranfer error");
    transfer(ctx.accounts.deposit_ctx(), args.amount)?;

    // set deposit_order_account
    let deposit_order_account = &mut ctx.accounts.deposit_order_account;
    deposit_order_account.token_amount = args.amount;
    deposit_order_account.pool_number = args.pool_index;
    deposit_order_account.actual_amount = args.amount;

    //set user_config
    let user_config = &mut ctx.accounts.user_config;
    user_config.bump = ctx.bumps.user_config;
    user_config.deposit_time = time as i64;
    user_config.amount += args.amount;

    let refunding_amount = get_refunding_amount(&pool_config, user_config);
    let offering_amount = get_offering_amount(&pool_config, user_config);
    let refund_amount = get_refunding_amount(&pool_config, user_config);

    user_config.raised_amount = refunding_amount;
    user_config.offering_amount = offering_amount;
    user_config.refund_amount = refund_amount;

    // update pool_config
    let pool_config = &mut ctx.accounts.pool_config;
    pool_config.total_amount = pool_config.total_amount.checked_add(args.amount).unwrap();
    pool_config.total_deposit_amount += 1;
    if !pool_config
        .address_list
        .contains(&ctx.accounts.signer.key())
    {
        pool_config.address_list.push(ctx.accounts.signer.key())
    }

    Ok(())
}
