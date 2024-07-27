use anchor_lang::prelude::*;

#[event]
pub struct AddLiquidityEvent {
    pub receiver: Pubkey,
    pub owner: Pubkey,
    pub token_id: u16,
    pub amount: u64,
}

#[event]
pub struct SendTokenEvent {
    pub receiver: Pubkey,
    pub user: Pubkey,
    pub token_id: u16,
    pub amount: u64,
}

#[event]
pub struct MessageReceivedEvent {
    pub vault: Pubkey,
    pub to_address: Pubkey,
    pub token_id: u16,
    pub amount: u64,
}

#[event]
pub struct WithdrawEvent {
    pub amount: u64,
    pub beneficiary: Pubkey,
}

#[event]
pub struct WithdrawTokenEvent {
    pub vault: Pubkey,
    pub to_address: Pubkey,
    pub token_id: u16,
    pub amount: u64,
}


#[event]
pub struct AddTokenEvent {
    pub token_id: u16,
    pub token_mint: Pubkey,
}

#[event]
pub struct RemoveTokenEvent {
    pub token_id: u16,
}
