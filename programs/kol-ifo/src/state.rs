use crate::errors::KolIfoError;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;
use libsecp256k1;
use solana_program::{keccak, pubkey::Pubkey};
#[account]
pub struct IFOConfig {
    pub bump: u8,
    //  ifo super admin
    pub admin: Pubkey,
    pub pool_number: u128,
    //deposit func use
    pub manager: [u8; 64],
}

impl IFOConfig {
    //check admin
    pub fn check_admin(&self, owner: Pubkey) -> Result<()> {
        require_keys_eq!(owner, self.admin, KolIfoError::AdminError);
        Ok(())
    }
}

#[account]
pub struct PoolConfig {
    pub bump: u8,
    pub start_time: u64,
    pub end_time: u64,
    pub claim_time: u64,
    pub min_amount: u64,
    pub max_amount: u64,
    pub raising_amount: u64,
    pub offering_amount: u64,
    pub total_amount: u64,
    pub over_funding: bool,
    //offering token
    pub offering_token_mint: Option<Pubkey>,
    //raising lp token
    pub lp_token_mint: Option<Pubkey>,
    pub user_count: u128,
    pub total_claim_order: u128,
    pub total_deposit_amount: u128,
    pub address_list: Vec<Pubkey>,
    pub is_refund: bool,    // Seven days refund
    pub initial_rate: u128, // 1e6
    pub tn: u128,
    pub cliff: u128,
    pub period: u128,
}

pub fn only_signer(
    signature: [u8; 64],
    message: Vec<u8>,
    recovery_id: u8,
    manager: [u8; 64],
) -> Result<()> {
    let message_hash = {
        let mut hasher = keccak::Hasher::default();
        hasher.hash(&message);
        hasher.result()
    };

    {
        let signature = libsecp256k1::Signature::parse_standard_slice(&signature)
            .map_err(|_| ProgramError::InvalidArgument)
            .unwrap();

        if signature.s.is_high() {
            msg!("signature with high-s value");
        }
    }
    let recovered_pubkey = secp256k1_recover(&message_hash.0, recovery_id, &signature)
        .map_err(|_| ProgramError::InvalidArgument)?;
    msg!("recovered public key{:?}",recovered_pubkey.0);
    require!(recovered_pubkey.0 == manager, KolIfoError::InvalidPublicKey);

    Ok(())
}

#[account]
#[derive(InitSpace)]
pub struct UserConfig {
    pub bump: u8,
    pub amount: u64,
    pub deposit_time: i64,
    pub claimed_amount: u128,
    pub raised_amount: u128,
    pub offering_amount: u128,
    pub refund_amount: u128,
    pub is_claimed: bool,
}

#[account]
#[derive(InitSpace)]
pub struct ClaimOrderConfig {
    pub bump: u8,
    pub user_account: Pubkey,
    pub pool_config_pda: Pubkey,
    pub offering_amount: u64,
    pub refund_amount: u64,
}

#[account]
#[derive(InitSpace)]
pub struct DepositOrder {
    pub bump: u8,
    pub token_amount: u64,
    pub pool_number: u128,
    pub actual_amount: u64,
}

#[account]
#[derive(InitSpace)]
pub struct PoolSolAccount {
    pub bump: u8,
    pub token_amount: u64,
}
