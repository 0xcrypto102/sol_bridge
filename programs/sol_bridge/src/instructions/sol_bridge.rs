use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ self, Mint, Token, TokenAccount, Transfer }
  };
use crate::{state::*, constants::*, error::*, event::*};
use solana_program::{program::invoke, system_instruction};

use pyth_solana_receiver_sdk::price_update::{PriceUpdateV2};
use pyth_solana_receiver_sdk::price_update::get_feed_id_from_hex;

pub fn add_liquidity(ctx: Context<AddLiquidity>, token_id: u16, target_chain_selector: u32,amount: u64) -> Result<()> {
    let bridge = &ctx.accounts.bridge;
    let user = &ctx.accounts.user;

    require!(bridge.owner == *ctx.accounts.user.key, BridgeErrorCode::InvalidOwner);

    // Check if token is supported
    require!(bridge.token_ids.contains(&token_id), BridgeErrorCode::UnsupportedToken);

    // Get the token address
    let token_address = bridge.get_token_address(token_id,target_chain_selector).ok_or(BridgeErrorCode::UnsupportedToken)?;


    require!(token_address == &ctx.accounts.token_mint.key(), BridgeErrorCode::DisMatchToken);


    let token_program = &ctx.accounts.token_program;
    let token_account = &ctx.accounts.token_account;
    let bridge_token_account = &ctx.accounts.bridge_token_account;

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
        receiver: ctx.accounts.bridge_token_account.key(),
        owner: user.key(),
        token_id,
        amount,
    });

    Ok(())
}

pub fn send(ctx: Context<Send>, token_id: u16, target_chain_selector: u32, amount: u64) -> Result<()> {
    let accts = ctx.accounts;

    // Check if token is supported
    require!(accts.bridge.token_ids.contains(&token_id), BridgeErrorCode::UnsupportedToken);

    // Get the token address
    let token_address = accts.bridge.get_token_address(token_id, target_chain_selector).ok_or(BridgeErrorCode::UnsupportedToken)?;


    require!(token_address == &accts.token_mint.key(), BridgeErrorCode::DisMatchToken);

    let target_balance = accts.bridge.get_target_balance(token_id, target_chain_selector)?;

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

    let price_update = &accts.price_update;
    // get_price_no_older_than will fail if the price update is more than 30 seconds old
    let maximum_age: u64 = 30;
    // get_price_no_older_than will fail if the price update is for a different price feed.
    // This string is the id of the BTC/USD feed. See https://pyth.network/developers/price-feed-ids for all available IDs.
    let feed_id: [u8; 32] = get_feed_id_from_hex("0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d")?;
    let price = price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &feed_id)?;
    msg!("The price is ({} ± {}) * 10^{}", price.price, price.conf, price.exponent);
    // 2-Format display values rounded to nearest dollar
    let sol_amount = ((accts.bridge.protocol_fee as u128) * (1000000000000 as u128) / (u64::try_from(price.price).unwrap() as u128) * (1000000000 as u128)) as u64;
    msg!("The sol amount is {}", sol_amount / 1000);

    // transfer protocol fee to vault address
    invoke(
        &system_instruction::transfer(
            &accts.user.key(),
            &accts.vault.key(),
            sol_amount / 1000
        ),
        &[
            accts.user.to_account_info().clone(),
            accts.vault.clone(),
            accts.system_program.to_account_info().clone(),
        ],
    )?;

    // Emit event
    emit!(SendTokenEvent {
        receiver: accts.bridge_token_account.key(),
        user: accts.user.key(),
        token_id,
        amount,
    });

    Ok(())
}

pub fn message_receive(ctx: Context<MessageReceive>, token_id: u16, target_chain_selector: u32, amount: u64) -> Result<()> {
    let bridge = &ctx.accounts.bridge;
    
    require!(bridge.owner == *ctx.accounts.owner.key, BridgeErrorCode::InvalidOwner);

    // Check if token is supported
    require!(bridge.token_ids.contains(&token_id), BridgeErrorCode::UnsupportedToken);

    // Check if token is supported
    let token_address = bridge.get_token_address(token_id,target_chain_selector).ok_or(BridgeErrorCode::UnsupportedToken)?;


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
        vault: bridge_token_account.key(),
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

    // Add this account to any instruction Context that needs price data.
    pub price_update: Account<'info, PriceUpdateV2>,

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
