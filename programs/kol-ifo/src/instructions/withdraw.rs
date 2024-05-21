use crate::seeds::{IFO_CONFIG_SEED, POOL_CONFIG_SEED, POOL_SOL_TOKEN_ACCOUNT};
use crate::state::PoolConfig;
use crate::{errors::KolIfoError, state::IFOConfig};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct WithdrawArgs {
    pub offer_amount: u64,
    pub lp_amount: u64,
    pub pool_index: u128,
}

#[derive(Accounts)]
#[instruction(args: WithdrawArgs)]
pub struct Withdraw<'info> {
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

impl<'info> Withdraw<'info> {
    pub fn withdraw_lp_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
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
pub fn withdraw_handler(ctx: Context<Withdraw>, args: WithdrawArgs) -> Result<()> {
    let ifo_config: &mut Box<Account<IFOConfig>> = &mut ctx.accounts.ifo_config;
    ifo_config.check_admin(ctx.accounts.signer.key())?;

    //check params
    let pool_config = &ctx.accounts.pool_config;
    if pool_config.lp_token_mint.is_some() {
        let pool_lp_token_account: &Box<Account<TokenAccount>> = &<Option<
            Box<anchor_lang::prelude::Account<'_, anchor_spl::token::TokenAccount>>,
        > as Clone>::clone(
            &ctx.accounts.pool_lp_token_account,
        )
        .unwrap();
        require!(
            args.lp_amount <= pool_lp_token_account.amount,
            KolIfoError::ParamError
        );
    }

    require!(
        args.offer_amount <= pool_config.offering_amount,
        KolIfoError::ParamError
    );

    let binding = args.pool_index.to_be_bytes();
    let seeds = &[
        POOL_CONFIG_SEED,
        &binding.as_ref(),
        &[ctx.accounts.pool_config.bump],
    ];

    // transfer offer amount
    if args.offer_amount > 0 {
        if ctx.accounts.pool_config.offering_token_mint.is_some() {
            //transfer spl
            transfer(
                ctx.accounts.withdraw_lp_ctx().with_signer(&[&seeds[..]]),
                args.offer_amount,
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

            anchor_lang::system_program::transfer(cpi_context, args.offer_amount)?;
        }
    }
    // transfer lp amount
    if args.lp_amount > 0 {
        if ctx.accounts.pool_config.lp_token_mint.is_some() {
            //transfer spl
            transfer(
                ctx.accounts.withdraw_lp_ctx().with_signer(&[&seeds[..]]),
                args.lp_amount,
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

            anchor_lang::system_program::transfer(cpi_context, args.lp_amount)?;
        }
    }

    Ok(())
}
