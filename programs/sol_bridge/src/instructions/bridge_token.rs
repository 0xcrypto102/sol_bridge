use anchor_lang::prelude::*;

use crate::{state::*, constants::*, error::*, event::*};

pub fn add_token(
  ctx: Context<ManageToken>,  
  local_token: Pubkey,        // Local token address (on Solana)
  remote_chain_selector: u64, // EVM chain selector (uint64)
  remote_token: String
) -> Result<()> {
  let bridge = &mut ctx.accounts.bridge;
  require!(bridge.owner == *ctx.accounts.owner.key, BridgeErrorCode::InvalidOwner);
  let token_id = bridge.add_token(local_token, remote_chain_selector, remote_token.clone())?;

  // Emit event
  emit!(AddTokenEvent {
    local_token,
    remote_chain_selector,
    remote_token,
    token_id
  });

  Ok(())
}


pub fn remove_token(
  ctx: Context<ManageToken>,
  local_token: Pubkey,        // Local token address (on Solana)
  remote_chain_selector: u64, // EVM chain selector (uint64)
  remote_token: String
) -> Result<()> {
  let bridge = &mut ctx.accounts.bridge;
  require!(bridge.owner == *ctx.accounts.owner.key, BridgeErrorCode::InvalidOwner);
  let token_id = bridge.remove_token(local_token, remote_chain_selector, remote_token)?;

  // Emit event
  emit!(RemoveTokenEvent {
    token_id,
    local_token
  });

  Ok(())
}

pub fn update_token_balance(ctx: Context<ManageToken>, local_token: Pubkey, remote_chain_selector: u64, remote_token: String, amount: u64, flag: bool) -> Result<()> {
  let bridge = &mut ctx.accounts.bridge;
  require!(bridge.owner == *ctx.accounts.owner.key, BridgeErrorCode::InvalidOwner);
  // Update the target balance
  let _target_balance = bridge.update_balance(local_token, remote_chain_selector, remote_token, amount, flag);

  Ok(())
}

pub fn get_token_id(
  ctx: Context<ManageToken>,  
  local_token: Pubkey,        // Local token address (on Solana)
  remote_chain_selector: u64, // EVM chain selector (uint64)
  remote_token: String
) -> Result<String> {
  let bridge = &mut ctx.accounts.bridge;

  // Encode local_token as bytes
  let binding = local_token.to_string();
  let local_token_bytes = binding.as_bytes();

  let token_id = bridge.get_token_id(
    local_token_bytes,       // Local token as bytes
    bridge.chain_selector,      // Solana chain selector (from Bridge struct)
    remote_chain_selector,    // EVM chain selector
    remote_token.as_bytes()   // Remote token as bytes
  )?;

  Ok(token_id)
}

#[derive(Accounts)]
pub struct ManageToken<'info> {
  #[account(mut)]
  pub owner: Signer<'info>,

  #[account(
      mut,
      seeds = [BRIDGE_SEED],
      bump
  )]
  pub bridge: Box<Account<'info, Bridge>>,
}
