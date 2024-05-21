use crate::state::{PoolConfig, UserConfig};
use anchor_lang::prelude::*;

use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use super::get_claim_amount::{GetClaimAmount, GetClaimAmountArgs};
use super::get_user_allocation;

pub fn get_offering_amount_handler(
    ctx: Context<GetClaimAmount>,
    _args: GetClaimAmountArgs,
) -> Result<u128> {
    let pool_config = &ctx.accounts.pool_config;
    let user_config = &ctx.accounts.user_config;
    let offering_amount = get_offering_amount(pool_config, user_config);
    Ok(offering_amount)
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
