use anchor_lang::prelude::*;

use crate::{state::*, constants::*, error::*, event::*};

pub fn add_token(ctx: Context<ManageToken>, token_id: u16, token_mint: Pubkey) -> Result<()> {
  let bridge = &mut ctx.accounts.bridge;
  require!(bridge.owner == *ctx.accounts.owner.key, BridgeErrorCode::InvalidOwner);
  let _ = bridge.add_token(token_id, token_mint);

  // Emit event
  emit!(AddTokenEvent {
      token_id: token_id,
      token_mint: token_mint,
  });

  Ok(())
}


pub fn remove_token(ctx: Context<ManageToken>, token_id: u16) -> Result<()> {
  let bridge = &mut ctx.accounts.bridge;
  require!(bridge.owner == *ctx.accounts.owner.key, BridgeErrorCode::InvalidOwner);
  bridge.remove_token(token_id);

  // Emit event
  emit!(RemoveTokenEvent {
      token_id: token_id,
  });

  Ok(())
}

pub fn update_token_balance(ctx: Context<ManageToken>, token_id: u16, token_amount: u64, flag: bool) -> Result<()> {
  let bridge = &mut ctx.accounts.bridge;
  require!(bridge.owner == *ctx.accounts.owner.key, BridgeErrorCode::InvalidOwner);
  // Update the target balance
  let target_balance = bridge.update_balance(token_id, token_amount, flag);

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
