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
