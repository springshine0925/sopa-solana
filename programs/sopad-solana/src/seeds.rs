use anchor_lang::prelude::*;

#[constant]
pub const IFO_CONFIG_SEED: &[u8] = b"ifo-config";

#[constant]
pub const POOL_CONFIG_SEED: &[u8] = b"pool-config";

#[constant]
pub const POOL_TOKEN_ACCOUNT_SEED: &[u8] = b"pool-token-account-seed";

#[constant]
pub const DEPOSIT_ORDER_SEED: &[u8] = b"deposit-order";

#[constant]
pub const USER_CONFIG_SEED: &[u8] = b"user-config";

#[constant]
pub const USER_TOKEN_ACCOUNT_SEED: &[u8] = b"user-token-account";

#[constant]
pub const CLAIM_ORDER_SEED: &[u8] = b"claim-order-seed";

#[constant]
pub const DEPOSIT_TYPEHASH: &[u8] =
    b"0x5389a5e529de40c9685335fe495d7d2f0e57ff19979a6e0484a4ebf599b6f2d4";

#[constant]
pub const POOL_SOL_TOKEN_ACCOUNT: &[u8] = b"pool-sol-token-account";
