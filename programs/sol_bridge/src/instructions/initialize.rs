use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ self, Mint, Token, TokenAccount, Transfer }
  };
use crate::{state::*, constants::*, error::*, event::*};
use solana_program::{program::invoke_signed, system_instruction};

use std::mem::size_of;

pub fn initialize(ctx: Context<Initialize>, protocol_fee: u64, chain_selecotr: u64) -> Result<()> {
    let accts = ctx.accounts;
    accts.bridge.owner = accts.owner.key();
    accts.bridge.protocol_fee = protocol_fee;
    accts.bridge.chain_selector = chain_selecotr;
    accts.bridge.vault = accts.vault.key();
    
    Ok(())
}

pub fn set_protocol_fee(ctx: Context<SetProtocolFee>, protocol_fee: u64) -> Result<()> {
    let bridge = &mut ctx.accounts.bridge;
    require!(bridge.owner == *ctx.accounts.owner.key, BridgeErrorCode::InvalidOwner);
    require!(protocol_fee != 0, BridgeErrorCode::InvalidProtocolFee);
    bridge.protocol_fee = protocol_fee;
    Ok(())
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let accts = ctx.accounts;

    require!(accts.bridge.owner == accts.owner.key(), BridgeErrorCode::InvalidOwner);

    let lamports = accts.vault.to_account_info().lamports();
    require!(amount <= lamports, BridgeErrorCode::InsufficientBalance);

    let (_, bump) = Pubkey::find_program_address(&[VAULT_SEED], &crate::ID);

    invoke_signed(
        &system_instruction::transfer(&accts.vault.key(), &accts.beneficiary.key(), amount),
        &[
            accts.vault.to_account_info().clone(),
            accts.beneficiary.to_account_info().clone(),
            accts.system_program.to_account_info().clone(),
        ],
        &[&[VAULT_SEED, &[bump]]],
    )?;

    emit!(WithdrawEvent {
        beneficiary: accts.beneficiary.key(),
    });

    Ok(())
}

pub fn withdraw_token(ctx: Context<WithdrawToken>, token_id: String, amount: u64) -> Result<()> {
    let bridge = &ctx.accounts.bridge;

    require!(bridge.owner == *ctx.accounts.owner.key, BridgeErrorCode::InvalidOwner);

    // Check if token is supported
    require!(bridge.token_ids.contains(&token_id), BridgeErrorCode::UnsupportedToken);

    // Get the token address
    let token_mint = bridge.get_token_address(token_id).ok_or(BridgeErrorCode::UnsupportedToken)?;

    require!(token_mint == &ctx.accounts.token_mint.key(), BridgeErrorCode::DisMatchToken);

    let token_program = &ctx.accounts.token_program;
    let bridge_token_account = &ctx.accounts.bridge_token_account;
    let beneficiary_token_account = &ctx.accounts.beneficiary_token_account;

    let balance = bridge_token_account.amount;
    require!(amount <= balance, BridgeErrorCode::InsufficientBalance);

    let (_, bump) = Pubkey::find_program_address(&[BRIDGE_SEED], ctx.program_id);
    let vault_seeds = &[BRIDGE_SEED, &[bump]];
    let signer = &[&vault_seeds[..]];

    // Transfer tokens from bridge to beneficiary
    let cpi_accounts = Transfer {
        from: bridge_token_account.to_account_info(),
        to: beneficiary_token_account.to_account_info(),
        authority: ctx.accounts.bridge.to_account_info(),
    };
    let cpi_context = CpiContext::new(token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_context.with_signer(signer), amount)?;

    emit!(
        WithdrawTokenEvent {
            token: *token_mint,
            amount,
        }
    );

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = owner, 
        seeds = [BRIDGE_SEED],
        bump,
        space = 5000
    )]
    pub bridge: Box<Account<'info, Bridge>>,
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump
    )]
    pub vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetProtocolFee<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [BRIDGE_SEED],
        bump
    )]
    pub bridge: Box<Account<'info, Bridge>>,
}


#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [BRIDGE_SEED],
        bump
    )]
    pub bridge: Box<Account<'info, Bridge>>,
    
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump
    )]
    pub vault: AccountInfo<'info>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub beneficiary: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct WithdrawToken<'info> {
    #[account(mut)]
    pub bridge: Account<'info, Bridge>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [BRIDGE_TOKEN_VAULT_SEED, token_mint.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = bridge
    )]
    pub bridge_token_account: Box<Account<'info, TokenAccount>>,
 
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = token_mint,
        associated_token::authority = owner
    )]
    pub beneficiary_token_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}