use anchor_lang::prelude::*;

#[error_code]
pub enum BridgeErrorCode {
    #[msg("Invalid owner.")]
    InvalidOwner,
    #[msg("Invalid chain selector.")]
    InvalidChainSelector,
    #[msg("Invalid protocol fee.")]
    InvalidProtocolFee,
    #[msg("Unsupported token.")]
    UnsupportedToken,
    #[msg("The token address is not matched.")]
    DisMatchToken,
    #[msg("Insufficient balance.")]
    InsufficientBalance,
    #[msg("Invalid message type.")]
    InvalidMessageType,
    #[msg("The token is already existed.")]
    AlreadyExist,
    #[msg("The price feed is invalid.")]
    InvalidPriceFeed
}
