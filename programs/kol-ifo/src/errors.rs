use anchor_lang::prelude::*;

#[error_code]
pub enum KolIfoError {
    #[msg("admin error")]
    AdminError,
    #[msg("param error")]
    ParamError,
    #[msg("time error:max_amount < min_amount")]
    TimeError1,
    #[msg("time error:endtime < start_time")]
    TimeError2,
    #[msg("time error:claim_time < end_time")]
    TimeError3,
    #[msg("raising_amount < 0")]
    RaisingAmountError,

    #[msg("token mint error")]
    TokenMintError,
    #[msg("fee receiver error")]
    FeeReceiverError,
    #[msg("Pool has been started")]
    PoolHasStarted,
    #[msg("Already claimed")]
    AlreadyClaimed,
    #[msg("Port3IFO: invalid signer")]
    InvalidSigner,
    #[msg("Seven-day refund is not supported")]
    RefundisNotSupported,
    #[msg("InvalidPublicKey")]
    InvalidPublicKey,
}
