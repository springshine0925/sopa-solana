use crate::seeds::{IFO_CONFIG_SEED, POOL_CONFIG_SEED, POOL_SOL_TOKEN_ACCOUNT, USER_CONFIG_SEED};
use crate::state::{PoolConfig, UserConfig};
use crate::{errors::SoPadError, state::IFOConfig};
use crate::{get_offering_amount, get_refunding_amount};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct RefundArgs {
    pub pool_index: u128,
}

#[derive(Accounts)]
#[instruction(args: RefundArgs)]
pub struct Refund<'info> {
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

    #[account(
        mut,
        seeds = [USER_CONFIG_SEED,signer.key().as_ref()],
        bump
    )]
    pub user_config: Box<Account<'info, UserConfig>>,

    /// CHECK: This account is not read or written
    #[account()]
    pub lp_token_mint: Option<Box<Account<'info, Mint>>>,

    #[account(
        init_if_needed,
        payer=signer,
        associated_token::mint = lp_token_mint,
        associated_token::authority = signer,
    )]
    pub user_lp_token_account: Option<Box<Account<'info, TokenAccount>>>,

    #[account(
        mut,
        seeds = [POOL_SOL_TOKEN_ACCOUNT],
        bump
    )]
    pub pool_token_account: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::mint = lp_token_mint,
        associated_token::authority = pool_config,
    )]
    pub pool_lp_token_account: Option<Box<Account<'info, TokenAccount>>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Refund<'info> {
    pub fn refund_lp_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let from=<Option<Box<anchor_lang::prelude::Account<'_, anchor_spl::token::TokenAccount>>> as Clone>::clone(&self.pool_lp_token_account).unwrap().to_account_info();
        let to=<Option<Box<anchor_lang::prelude::Account<'_, anchor_spl::token::TokenAccount>>> as Clone>::clone(&self.user_lp_token_account).unwrap().to_account_info();
        let cpi_accounts = Transfer {
            from: from,
            to: to,
            authority: self.pool_config.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
pub fn refund_handler(ctx: Context<Refund>, args: RefundArgs) -> Result<()> {
    let clock = Clock::get().unwrap();
    let time = clock.unix_timestamp as u64;

    require!(
        ctx.accounts.pool_config.is_refund,
        SoPadError::RefundisNotSupported
    );
    require!(
        ctx.accounts.user_config.claimed_amount == 0,
        SoPadError::ParamError
    );
    require!(
        ctx.accounts.pool_config.end_time < time,
        SoPadError::ParamError
    );
    require!(
        time - ctx.accounts.pool_config.end_time < 7 * 86400,
        SoPadError::ParamError
    );

    let amount = ctx.accounts.user_config.amount;
    // let amount=63;

    //update user config
    let user_config = &mut ctx.accounts.user_config;
    user_config.amount = 0;

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

    // transfer  amount
    if ctx.accounts.pool_config.offering_token_mint.is_some() {
        //transfer spl
        transfer(
            ctx.accounts.refund_lp_ctx().with_signer(&[&seeds[..]]),
            amount,
        )?;
    } else {
        let bump_seed = ctx.bumps.pool_token_account;
        let sol_seed: &[&[&[u8]]] = &[&[POOL_SOL_TOKEN_ACCOUNT, &[bump_seed]]];
        //transfer sol
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.pool_token_account.to_account_info(),
                to: ctx.accounts.signer.to_account_info(),
            },
        )
        .with_signer(sol_seed);

        anchor_lang::system_program::transfer(cpi_context, amount)?;
    }

    Ok(())
}
