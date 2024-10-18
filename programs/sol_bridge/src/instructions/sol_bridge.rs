use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ self, Mint, Token, TokenAccount, Transfer }
  };
use crate::{state::*, constants::*, error::*, event::*};
use solana_program::{program::invoke, system_instruction};

pub fn add_liquidity(ctx: Context<AddLiquidity>, amount: u64, remote_chain_selector: u64, remote_token: String) -> Result<()> {
    let accts = ctx.accounts;

    let bridge = &accts.bridge;
    let user = &accts.user;
    let local_token = accts.token_mint.key();

    require!(bridge.owner == user.key(), BridgeErrorCode::InvalidOwner);

    // Encode local_token as bytes
    let binding = local_token.to_string();
    let local_token_bytes = binding.as_bytes();

    let token_id = bridge.get_token_id(
        local_token_bytes,
        bridge.chain_selector,
        remote_chain_selector,
        remote_token.as_bytes()
    )?;

    // Check if token is supported
    require!(bridge.token_ids.contains(&token_id), BridgeErrorCode::UnsupportedToken);

    // Get the token address
    let token_address = bridge.get_token_address(token_id).ok_or(BridgeErrorCode::UnsupportedToken)?;

    require!(token_address == &local_token, BridgeErrorCode::DisMatchToken);

    let token_program = &accts.token_program;
    let token_account = &accts.token_account;
    let bridge_token_account = &accts.bridge_token_account;

    // Transfer tokens from user to bridge
    let cpi_accounts = Transfer {
        from: token_account.to_account_info(),
        to: bridge_token_account.to_account_info(),
        authority: user.to_account_info(),
    };
    let cpi_context = CpiContext::new(token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_context, amount)?;

    // Emit event
    emit!(AddLiquidityEvent {
        local_token,
        amount,
        remote_chain_selector,
        remote_token,
    });

    Ok(())
}

pub fn send(
    ctx: Context<Send>, 
    amount: u64, 
    remote_bridge: String,
    remote_chain_selector: u64, 
    remote_token: String
) -> Result<()> {
    let accts = ctx.accounts;
    let local_token = accts.token_mint.key();

    // Encode local_token as bytes
    let binding = local_token.to_string();
    let local_token_bytes = binding.as_bytes();

    let token_id = accts.bridge.get_token_id(
        local_token_bytes,
        accts.bridge.chain_selector,
        remote_chain_selector,
        remote_token.as_bytes()
    )?;

    // Check if token is supported
    require!(accts.bridge.token_ids.contains(&token_id.clone()), BridgeErrorCode::UnsupportedToken);

    // Get the token address
    let token_address = accts.bridge.get_token_address(token_id.clone()).ok_or(BridgeErrorCode::UnsupportedToken)?;


    require!(token_address == &local_token, BridgeErrorCode::DisMatchToken);

    let target_balance = accts.bridge.get_target_balance(token_id)?;

    require!(target_balance > amount, BridgeErrorCode::InsufficientBalance);

    let token_program = &accts.token_program;
    let token_account = &accts.token_account;
    let bridge_token_account = &accts.bridge_token_account;

    // Transfer tokens from user to bridge
    let cpi_accounts = Transfer {
        from: token_account.to_account_info(),
        to: bridge_token_account.to_account_info(),
        authority: accts.user.to_account_info(),
    };
    let cpi_context = CpiContext::new(token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_context, amount)?;

    // 2-Format display values rounded to nearest dollar
    let sol_amount = accts.bridge.protocol_fee;

    // transfer protocol fee to vault address
    invoke(
        &system_instruction::transfer(
            &accts.user.key(),
            &accts.vault.key(),
            sol_amount
        ),
        &[
            accts.user.to_account_info().clone(),
            accts.vault.clone(),
            accts.system_program.to_account_info().clone(),
        ],
    )?;

    // Emit event
    emit!(SendTokenEvent {
        local_token,
        amount,
        remote_bridge,
        remote_chain_selector,
        remote_token
    });

    Ok(())
}

pub fn message_receive(ctx: Context<MessageReceive>, token_id: String, source_chain_selector: u64, amount: u64) -> Result<()> {
    let bridge = &ctx.accounts.bridge;
    
    require!(bridge.owner == *ctx.accounts.owner.key, BridgeErrorCode::InvalidOwner);

    // Check if token is supported
    require!(bridge.token_ids.contains(&token_id.clone()), BridgeErrorCode::UnsupportedToken);

    // Check if token is supported
    let token_address = bridge.get_token_address(token_id.clone()).ok_or(BridgeErrorCode::UnsupportedToken)?;

    require!(token_address == &ctx.accounts.token_mint.key(), BridgeErrorCode::DisMatchToken);

    let token_program = &ctx.accounts.token_program;
    let bridge_token_account = &ctx.accounts.bridge_token_account;
    let to_token_account = &ctx.accounts.user_token_account;

    let balance = bridge_token_account.amount;
    require!(amount <= balance, BridgeErrorCode::InsufficientBalance);

    let (_, bump) = Pubkey::find_program_address(&[BRIDGE_SEED], ctx.program_id);
    let vault_seeds = &[BRIDGE_SEED, &[bump]];
    let signer = &[&vault_seeds[..]];

    // Transfer tokens from bridge to receiver
    let cpi_accounts = Transfer {
        from: bridge_token_account.to_account_info(),
        to: to_token_account.to_account_info(),
        authority: ctx.accounts.bridge.to_account_info(),
    };

    let cpi_context = CpiContext::new(token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_context.with_signer(signer), amount)?;


    emit!(MessageReceivedEvent {
        source_chain_selector,
        to_address: to_token_account.key(),
        token_id,
        amount,
    });

    Ok(())
}


#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [BRIDGE_SEED],
        bump
    )]
    pub bridge: Box<Account<'info, Bridge>>,

    #[account(mut)]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        seeds = [BRIDGE_TOKEN_VAULT_SEED, token_mint.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = bridge
    )]
    pub bridge_token_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Send<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

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

    #[account(mut)]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [BRIDGE_TOKEN_VAULT_SEED, token_mint.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = bridge
    )]
    pub bridge_token_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MessageReceive<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [BRIDGE_SEED],
        bump
    )]
    pub bridge: Box<Account<'info, Bridge>>,

    #[account(mut)]
    pub token_mint: Box<Account<'info, Mint>>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub user: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = token_mint,
        associated_token::authority = user
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [BRIDGE_TOKEN_VAULT_SEED, token_mint.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = bridge
    )]
    pub bridge_token_account: Box<Account<'info, TokenAccount>>,
 
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}
