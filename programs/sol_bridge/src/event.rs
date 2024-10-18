use anchor_lang::prelude::*;

#[event]
pub struct AddLiquidityEvent {
    pub local_token: Pubkey,
    pub amount: u64,
    pub remote_chain_selector: u64,
    pub remote_token: String,
}

#[event]
pub struct SendTokenEvent {
    pub local_token: Pubkey,
    pub amount: u64,
    pub remote_bridge: String,
    pub remote_chain_selector: u64,
    pub remote_token: String,
}

#[event]
pub struct MessageReceivedEvent {
    pub source_chain_selector: u64,
    pub to_address: Pubkey,
    pub token_id: String,
    pub amount: u64,
}

#[event]
pub struct WithdrawEvent {
    pub beneficiary: Pubkey,
}

#[event]
pub struct WithdrawTokenEvent {
    pub token: Pubkey,
    pub amount: u64,
}


#[event]
pub struct AddTokenEvent {
    pub local_token: Pubkey,
    pub remote_chain_selector: u64,
    pub remote_token: String,
    pub token_id: String,
}

#[event]
pub struct RemoveTokenEvent {
    pub token_id: String,
    pub local_token: Pubkey
}
