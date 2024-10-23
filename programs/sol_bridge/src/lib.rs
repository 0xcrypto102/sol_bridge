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

declare_id!("6gUrEYhacs6ZeHZFfDEBih1PRY7417vTYZjbfD62mkjV");

#[program]
pub mod sol_bridge {
    use super::*;
    // owner functions
    pub fn initialize(ctx: Context<Initialize>, protocol_fee: u64, chain_selecotr: u64) -> Result<()> {
        instructions::initialize(ctx, protocol_fee, chain_selecotr)
    }

    pub fn set_protocol_fee(ctx: Context<SetProtocolFee>, protocol_fee: u64) -> Result<()> {
        instructions::set_protocol_fee(ctx, protocol_fee)
    }

    pub fn withdraw_token(ctx: Context<WithdrawToken>, token_id: String, amount: u64) -> Result<()> {
        instructions::withdraw_token(ctx, token_id, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }

    pub fn add_token(
        ctx: Context<ManageToken>, 
        local_token: Pubkey, 
        remote_chain_selector: u64, 
        remote_token: String
    ) -> Result<()> {
        instructions::add_token(
            ctx, 
            local_token, 
            remote_chain_selector,
            remote_token
        )
    }

    pub fn remove_token(
        ctx: Context<ManageToken>, 
        local_token: Pubkey, 
        remote_chain_selector: u64, 
        remote_token: String
    ) -> Result<()> {
        instructions::remove_token(
            ctx, 
            local_token, 
            remote_chain_selector, 
            remote_token
        )
    }
  
    pub fn add_liquidity(
        ctx: Context<AddLiquidity>, 
        amount: u64, 
        remote_chain_selector: u64, 
        remote_token: String
    ) -> Result<()> {
        instructions::add_liquidity(
            ctx, 
            amount, 
            remote_chain_selector,
            remote_token
        )
    }

    pub fn message_receive(
        ctx: Context<MessageReceive>, 
        token_id: String, 
        source_chain_selector: u64, 
        amount: u64
    ) -> Result<()> {
        instructions::message_receive(
            ctx, 
            token_id, 
            source_chain_selector, 
            amount
        )
    }

    //  user function
    pub fn send(ctx: Context<Send>, 
        amount: u64, 
        remote_bridge: String,
        remote_chain_selector: u64, 
        remote_token: String
    ) -> Result<()> {
        instructions::send(
            ctx, 
            amount, 
            remote_bridge, 
            remote_chain_selector,
            remote_token
        )
    }
    
}
