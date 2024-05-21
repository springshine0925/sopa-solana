use crate::errors::SoPadError;
use crate::instructions::get_claim_amount::get_claim_amount;
use crate::instructions::get_offering_amount::get_offering_amount;
use crate::instructions::get_refunding_amount;
use crate::seeds::{CLAIM_ORDER_SEED, POOL_CONFIG_SEED, POOL_SOL_TOKEN_ACCOUNT, USER_CONFIG_SEED};
use crate::state::{ClaimOrderConfig, PoolConfig, UserConfig};
use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount},
};

use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use solana_program::clock::Clock;
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct UpdateUserArgs {
    pub pool_index: u128,
    pub user_account: Pubkey,
    pub pool_config_pda: Pubkey,
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
        mut,
        seeds =[USER_CONFIG_SEED,signer.key().as_ref()],
        bump=user_config.bump
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
    // Get current block timestamp
    let clock = Clock::get().unwrap();
    require!(
        clock.unix_timestamp as u64 > ctx.accounts.pool_config.claim_time,
        SoPadError::ParamError
    );

    let user_config = &ctx.accounts.user_config;
    require!(user_config.amount > 0, SoPadError::ParamError);

    let mut offering_token_amount =
        get_offering_amount(&ctx.accounts.pool_config, &ctx.accounts.user_config);
    let mut refunding_token_amount = 0;
    if user_config.claimed_amount == 0 {
        refunding_token_amount =
            get_refunding_amount(&ctx.accounts.pool_config, &ctx.accounts.user_config);
    }

    require!(
        user_config.claimed_amount < offering_token_amount,
        SoPadError::AlreadyClaimed
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

    // update user config
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
