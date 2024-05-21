use crate::state::{PoolConfig, UserConfig};
use anchor_lang::prelude::*;

use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use super::get_claim_amount::{GetClaimAmount, GetClaimAmountArgs};

pub fn get_refunding_amount_handler(
    ctx: Context<GetClaimAmount>,
    _args: GetClaimAmountArgs,
) -> Result<u128> {
    let pool_config = &ctx.accounts.pool_config;
    let user_config = &ctx.accounts.user_config;
    let refunding_amount = get_refunding_amount(pool_config, user_config);
    Ok(refunding_amount)
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
