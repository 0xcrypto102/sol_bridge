pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod event;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;
pub use state::*;
pub use event::*;

declare_id!("CYQLKQPkF4okEq3pLmT1rNHqQoePz1pDEA2aaWF2BC4E");

#[program]
pub mod sol_bridge {
    use super::*;
    // owner functions
    pub fn initialize(ctx: Context<Initialize>, protocol_fee: u64) -> Result<()> {
        instructions::initialize(ctx, protocol_fee)
    }

    pub fn set_protocol_fee(ctx: Context<SetProtocolFee>, protocol_fee: u64) -> Result<()> {
        instructions::set_protocol_fee(ctx, protocol_fee)
    }

    pub fn withdraw_token(ctx: Context<WithdrawToken>, token_id: u16, amount: u64) -> Result<()> {
        instructions::withdraw_token(ctx, token_id, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }

    pub fn add_token(ctx: Context<ManageToken>, token_id: u16, token_mint: Pubkey) -> Result<()> {
        instructions::add_token(ctx, token_id, token_mint)
    }

    pub fn remove_token(ctx: Context<ManageToken>, token_id: u16) -> Result<()> {
        instructions::remove_token(ctx, token_id)
    }

    pub fn update_token_balance(ctx: Context<ManageToken>, token_id: u16, amount: u64, flag: bool) -> Result<()> {
        instructions::update_token_balance(ctx, token_id, amount, flag)
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, token_id: u16, amount: u64) -> Result<()> {
        instructions::add_liquidity(ctx, token_id, amount)
    }

    pub fn message_receive(ctx: Context<MessageReceive>, token_id: u16, amount: u64) -> Result<()> {
        instructions::message_receive(ctx, token_id, amount)
    }

    //  user function
    pub fn send(ctx: Context<Send>, token_id: u16, amount: u64) -> Result<()> {
        instructions::send(ctx, token_id, amount)
    }
    
}
