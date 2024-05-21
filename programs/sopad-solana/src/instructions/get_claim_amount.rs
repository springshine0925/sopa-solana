use crate::seeds::{POOL_CONFIG_SEED, USER_CONFIG_SEED};
use crate::state::{PoolConfig, UserConfig};
use anchor_lang::prelude::*;

use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use solana_program::clock::Clock;

use super::get_offering_amount::get_offering_amount;

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
