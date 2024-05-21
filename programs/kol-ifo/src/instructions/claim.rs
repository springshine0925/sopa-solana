

use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use solana_program::clock::Clock;
use crate::errors::KolIfoError;
use crate::seeds::IFO_CONFIG_SEED;
use crate::seeds::{CLAIM_ORDER_SEED, POOL_CONFIG_SEED, POOL_SOL_TOKEN_ACCOUNT, USER_CONFIG_SEED};
use crate::state::only_signer;
use crate::state::IFOConfig;
use crate::state::{ClaimOrderConfig, PoolConfig, UserConfig};
use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount},
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct GetClaimAmountArgs {
    pub pool_index: u128,
}

#[derive(Accounts)]
#[instruction(args: GetClaimAmountArgs)]
pub struct GetClaimAmount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [POOL_CONFIG_SEED,args.pool_index.to_be_bytes().as_ref()],
        bump=pool_config.bump
    )]
    pub pool_config: Box<Account<'info, PoolConfig>>,

    #[account(
        mut,
        seeds = [USER_CONFIG_SEED,signer.key().as_ref()],
        bump=user_config.bump
    )]
    pub user_config: Box<Account<'info, UserConfig>>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UpdateUserArgs {
    pub pool_index: u128,
    pub user_account: Pubkey,
    pub pool_config_pda: Pubkey,
    pub signature: [u8; 64],
    pub message: Vec<u8>,
    pub recovery_id: u8,
}

#[derive(Accounts)]
#[instruction(args: UpdateUserArgs)]
pub struct UpdateUser<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [POOL_CONFIG_SEED,args.pool_index.to_be_bytes().as_ref()],
        bump=pool_config.bump
    )]
    pub pool_config: Box<Account<'info, PoolConfig>>,

    #[account(
        mut,
        seeds = [IFO_CONFIG_SEED],
        bump=ifo_config.bump
    )]
    pub ifo_config: Box<Account<'info, IFOConfig>>,

    //rasing token
    #[account(
        mut,
        seeds = [POOL_SOL_TOKEN_ACCOUNT],
        bump
    )]
    pub pool_token_account: SystemAccount<'info>,

    /// CHECK: This account is not read or written
    #[account()]
    pub lp_token_mint: Option<Box<Account<'info, Mint>>>,

    #[account(
        mut,
        associated_token::mint = lp_token_mint,
        associated_token::authority = signer,
    )]
    pub user_lp_token_account: Option<Box<Account<'info, TokenAccount>>>,

    #[account(
        mut,
        associated_token::mint = lp_token_mint,
        associated_token::authority = pool_config
    )]
    pub pool_lp_token_account: Option<Box<Account<'info, TokenAccount>>>,

    //offering token
    /// CHECK: This account is not read or written
    #[account()]
    pub offering_token_mint: Option<Box<Account<'info, Mint>>>,

    #[account(
        mut,
        associated_token::mint = offering_token_mint,
        associated_token::authority = signer,
    )]
    pub user_offering_token_account: Option<Box<Account<'info, TokenAccount>>>,

    #[account(
        mut,
        associated_token::mint = offering_token_mint,
        associated_token::authority = pool_config
    )]
    pub pool_offering_token_account: Option<Box<Account<'info, TokenAccount>>>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8+UserConfig::INIT_SPACE,
        seeds = [USER_CONFIG_SEED,signer.key().as_ref()],
        bump
    )]
    pub user_config: Box<Account<'info, UserConfig>>,

    #[account(
        init,
        payer=signer,
        space=8+ClaimOrderConfig::INIT_SPACE,
        seeds = [
            CLAIM_ORDER_SEED,
            pool_config.key().as_ref(),
            pool_config.total_claim_order.to_be_bytes().as_ref()
        ],
        bump,
    )]
    pub claim_order_account: Box<Account<'info, ClaimOrderConfig>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> UpdateUser<'info> {
    pub fn deposit_raising_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: <Option<Box<anchor_lang::prelude::Account<'_, anchor_spl::token::TokenAccount>>> as Clone>::clone(&self.pool_lp_token_account).unwrap().to_account_info(),
            to: <Option<Box<anchor_lang::prelude::Account<'_, anchor_spl::token::TokenAccount>>> as Clone>::clone(&self.user_lp_token_account).unwrap().to_account_info(),
            authority: self.pool_config.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn deposit_offering_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: <Option<Box<anchor_lang::prelude::Account<'_, anchor_spl::token::TokenAccount>>> as Clone>::clone(&self.pool_offering_token_account).unwrap().to_account_info(),
            to: <Option<Box<anchor_lang::prelude::Account<'_, anchor_spl::token::TokenAccount>>> as Clone>::clone(&self.user_offering_token_account).unwrap().to_account_info(),
            authority: self.pool_config.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    // allocation 100000 means 0.1(10%), 1 meanss 0.000001(0.0001%), 1000000 means 1(100%)
    pub fn get_user_allocation(&self) -> Decimal {
        let pool_config = &self.pool_config;
        let user_config = &self.user_config;
        let user_amount = Decimal::from_str(user_config.amount.to_string().as_str()).unwrap();
        let total_amount =
            Decimal::from_str(pool_config.total_amount.to_string().as_str()).unwrap();
        let allocation = user_amount
            .checked_mul(dec!(1e12))
            .unwrap()
            .checked_div(total_amount)
            .unwrap()
            .checked_div(dec!(1e6))
            .unwrap();

        return allocation;
    }
}

pub fn claim_pool_handler(ctx: Context<UpdateUser>, args: UpdateUserArgs) -> Result<()> {
    //only signer
    let ifo_config = &ctx.accounts.ifo_config;
    only_signer(
        args.signature,
        args.message,
        args.recovery_id,
        ifo_config.manager.clone(),
    )?;


    // Get current block timestamp
    let clock = Clock::get().unwrap();
    require!(
        clock.unix_timestamp as u64 > ctx.accounts.pool_config.claim_time,
        KolIfoError::ParamError
    );

    let user_config = &ctx.accounts.user_config;
    require!(user_config.amount > 0, KolIfoError::ParamError);

    let mut offering_token_amount =
        get_offering_amount(&ctx.accounts.pool_config, &ctx.accounts.user_config);
    let mut refunding_token_amount = 0;
    if user_config.claimed_amount == 0 {
        refunding_token_amount =
            get_refunding_amount(&ctx.accounts.pool_config, &ctx.accounts.user_config);
    }

    require!(
        user_config.claimed_amount < offering_token_amount,
        KolIfoError::AlreadyClaimed
    );

    let offering_token_amount_decimal =
        Decimal::from_str(offering_token_amount.to_string().as_str()).unwrap();
    offering_token_amount = get_claim_amount(
        &ctx.accounts.pool_config,
        &ctx.accounts.user_config,
        offering_token_amount_decimal,
    )
    .to_u128()
    .unwrap();

    // set user config
    let user_config = &mut ctx.accounts.user_config;
    user_config.claimed_amount += offering_token_amount;
    let refunding_amount = get_refunding_amount(&ctx.accounts.pool_config, user_config);
    let offering_amount = get_offering_amount(&ctx.accounts.pool_config, user_config);
    let refund_amount = get_refunding_amount(&ctx.accounts.pool_config, user_config);

    user_config.raised_amount = refunding_amount;
    user_config.offering_amount = offering_amount;
    user_config.refund_amount = refund_amount;

    let binding = args.pool_index.to_be_bytes();
    let seeds = &[
        POOL_CONFIG_SEED,
        &binding.as_ref(),
        &[ctx.accounts.pool_config.bump],
    ];
    let bump_seed = ctx.bumps.pool_token_account;
    let sol_seed: &[&[&[u8]]] = &[&[POOL_SOL_TOKEN_ACCOUNT, &[bump_seed]]];

    //transfer offering token
    if ctx.accounts.pool_config.offering_token_mint.is_some() {
        //transfer spl
        transfer(
            ctx.accounts
                .deposit_offering_ctx()
                .with_signer(&[&seeds[..]]),
            offering_token_amount.try_into().unwrap(),
        )?;
    }

    //transfer rasing token
    if refunding_token_amount > 0 {
        if ctx.accounts.pool_config.lp_token_mint.is_some() {
            //transfer spl
            transfer(
                ctx.accounts
                    .deposit_raising_ctx()
                    .with_signer(&[&seeds[..]]),
                refunding_token_amount.try_into().unwrap(),
            )?;
        } else {
            //transfer sol
            let cpi_context = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.pool_token_account.to_account_info(),
                    to: ctx.accounts.signer.to_account_info(),
                },
            )
            .with_signer(sol_seed);

            anchor_lang::system_program::transfer(
                cpi_context,
                refunding_token_amount.try_into().unwrap(),
            )?;
        }
    }

    //set claim order
    let claim_order_account = &mut ctx.accounts.claim_order_account;
    claim_order_account.user_account = args.user_account;
    claim_order_account.pool_config_pda = args.pool_config_pda;
    claim_order_account.refund_amount = refunding_token_amount as u64;
    claim_order_account.offering_amount = offering_token_amount as u64;

    Ok(())
}


pub fn get_claim_amount_handler(ctx: Context<GetClaimAmount>) -> Result<u128> {
    let pool_config = &ctx.accounts.pool_config;
    let user_config = &ctx.accounts.user_config;
    let offering_amount = get_offering_amount(pool_config, user_config);
    let claim_amount = get_claim_amount(pool_config, user_config, offering_amount.into());
    Ok(claim_amount.to_u128().unwrap())
}

pub fn get_claim_amount(
    pool_config: &Box<Account<PoolConfig>>,
    user_config: &Box<Account<UserConfig>>,
    offering_amount: Decimal,
) -> Decimal {
    if pool_config.tn == 0 {
        return offering_amount;
    }

    let pool_unlock_config_initial_rate =
        Decimal::from_str(pool_config.initial_rate.to_string().as_str()).unwrap();
    let pool_unlock_config_tn = Decimal::from_str(pool_config.tn.to_string().as_str()).unwrap();
    let user_config_claimed_amount =
        Decimal::from_str(user_config.claimed_amount.to_string().as_str()).unwrap();

    let clock = Clock::get().unwrap();
    let clock_timestamp = clock.unix_timestamp;

    if pool_config.initial_rate == 0 || pool_unlock_config_initial_rate.eq(&dec!(1e6)) {
        return offering_amount;
    }

    let mut claim_amount = dec!(0);
    if user_config.claimed_amount == 0 {
        claim_amount = offering_amount
            .checked_mul(pool_unlock_config_initial_rate)
            .unwrap()
            .checked_div(dec!(1e6))
            .unwrap();
    } else {
        claim_amount = offering_amount
            .checked_mul(dec!(1e6) - pool_unlock_config_initial_rate)
            .unwrap()
            .checked_div(pool_unlock_config_tn)
            .unwrap();
        let tmp_value = clock_timestamp
            .checked_sub(pool_config.claim_time.try_into().unwrap())
            .unwrap()
            .checked_sub(pool_config.cliff.try_into().unwrap())
            .unwrap()
            .checked_div(pool_config.period.try_into().unwrap())
            .unwrap();

        let tmp_value_decimal = Decimal::from_str(tmp_value.to_string().as_str()).unwrap();
        claim_amount = claim_amount
            .checked_mul(tmp_value_decimal.checked_div(dec!(1e6)).unwrap())
            .unwrap();
        claim_amount = claim_amount
            .checked_add(
                offering_amount
                    .checked_mul(pool_unlock_config_initial_rate)
                    .unwrap()
                    .checked_div(dec!(1e6))
                    .unwrap(),
            )
            .unwrap();
    }

    if claim_amount > user_config_claimed_amount {
        claim_amount = claim_amount
            .checked_sub(user_config_claimed_amount)
            .unwrap();
    }

    if claim_amount
        > offering_amount
            .checked_sub(user_config_claimed_amount)
            .unwrap()
    {
        claim_amount = offering_amount
            .checked_sub(user_config_claimed_amount)
            .unwrap();
    }

    return claim_amount;
}

pub fn get_offering_amount(
    pool_config: &Box<Account<PoolConfig>>,
    user_config: &Box<Account<UserConfig>>,
) -> u128 {
    let offering_amount =
        Decimal::from_str(pool_config.offering_amount.to_string().as_str()).unwrap();
    let raising_amount =
        Decimal::from_str(pool_config.raising_amount.to_string().as_str()).unwrap();
    if pool_config.offering_amount > pool_config.raising_amount {
        let allocation = get_user_allocation(pool_config, user_config);
        return (offering_amount
            .checked_mul(allocation)
            .unwrap()
            .checked_div(dec!(1e6))
            .unwrap())
        .to_u128()
        .unwrap();
    } else {
        let user_config_amount =
            Decimal::from_str(user_config.amount.to_string().as_str()).unwrap();
        return (user_config_amount
            .checked_mul(offering_amount)
            .unwrap()
            .checked_div(raising_amount)
            .unwrap())
        .to_u128()
        .unwrap();
    }
}

pub fn get_refunding_amount(
    pool_config: &Box<Account<PoolConfig>>,
    user_config: &Box<Account<UserConfig>>,
) -> u128 {
    if pool_config.total_amount <= pool_config.raising_amount {
        return 0;
    }
    let allocation = get_user_allocation(&pool_config, user_config);
    let rasising_amount =
        Decimal::from_str(pool_config.raising_amount.to_string().as_str()).unwrap();
    let pay_amount = rasising_amount
        .checked_mul(allocation)
        .unwrap()
        .checked_div(dec!(1e6))
        .unwrap();
    let user_config_amount = Decimal::from_str(user_config.amount.to_string().as_str()).unwrap();
    return (user_config_amount - pay_amount).to_u128().unwrap();
}

pub fn get_user_allocation(
    pool_config: &Box<Account<PoolConfig>>,
    user_config: &Box<Account<UserConfig>>,
) -> Decimal {
    let user_amount = Decimal::from_str(user_config.amount.to_string().as_str()).unwrap();
    let total_amount = Decimal::from_str(pool_config.total_amount.to_string().as_str()).unwrap();
    let allocation = user_amount
        .checked_mul(dec!(1e12))
        .unwrap()
        .checked_div(total_amount)
        .unwrap()
        .checked_div(dec!(1e6))
        .unwrap();

    return allocation;
}
