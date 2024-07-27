use anchor_lang::prelude::*;

use crate::error::BridgeErrorCode;

#[account]
#[derive(Default)]
pub struct Bridge {
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub protocol_fee: u64,
    pub token_ids: Vec<u16>,
    pub token_addresses: Vec<Pubkey>,
    pub target_balances: Vec<u64>,
}

impl Bridge {
    pub fn add_token(&mut self, token_id: u16, token_address: Pubkey) -> Result<()> {
        if self.token_ids.contains(&token_id) {
            return Err(BridgeErrorCode::AlreadyExist.into()); // Token ID already exists
        }
        self.token_ids.push(token_id);
        self.token_addresses.push(token_address);
        self.target_balances.push(0);
        Ok(())
    }

    pub fn remove_token(&mut self, token_id: u16) -> Result<()> {
        if let Some(index) = self.token_ids.iter().position(|&id| id == token_id) {
            self.token_ids.remove(index);
            self.token_addresses.remove(index);
            self.target_balances.remove(index);
            Ok(())
        } else {
            Err(BridgeErrorCode::UnsupportedToken.into()) // Token ID not found
        }
    }

    pub fn get_token_address(&self, token_id: u16) -> Option<&Pubkey> {
        self.token_ids.iter().position(|&id| id == token_id)
            .map(|index| &self.token_addresses[index])
    }

    pub fn get_target_balance(&self, token_id: u16) -> Result<u64> {
        if let Some(index) = self.token_ids.iter().position(|&id| id == token_id) {
            Ok(self.target_balances[index])
        } else {
            Err(BridgeErrorCode::UnsupportedToken.into()) // Token ID not found
        }
    }

    pub fn update_balance(&mut self, token_id: u16, amount: u64, flag: bool) -> Result<()> {
        if let Some(index) = self.token_ids.iter().position(|&id| id == token_id) {
            if flag {
                self.target_balances[index] += amount;
            } else {
                self.target_balances[index] -= amount;
            }
            Ok(())
        } else {
            Err(BridgeErrorCode::UnsupportedToken.into()) // Token ID not found
        }
    }
}