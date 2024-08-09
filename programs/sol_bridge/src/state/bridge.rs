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
    pub target_chain_selectors: Vec<u32>,
}

impl Bridge {
    pub fn add_token(&mut self, token_id: u16, target_chain_selector: u32, token_address: Pubkey, ) -> Result<()> {
        let mut flag = false;

        if let Some(index) = self.token_ids.iter().position(|&id| id == token_id) {
            if self.target_chain_selectors[index] != target_chain_selector {
                flag = true;
            }
        } else {
            flag = true;
        }

        if !flag {
            return Err(BridgeErrorCode::AlreadyExist.into()); // Token ID already exists
        }
        self.token_ids.push(token_id);
        self.token_addresses.push(token_address);
        self.target_balances.push(0);
        self.target_chain_selectors.push(target_chain_selector);
        
        Ok(())
    }

    pub fn remove_token(&mut self, token_id: u16, target_chain_selector: u32) -> Result<()> {
        if let Some(index) = self.token_ids.iter().position(|&id| id == token_id) {
            if self.target_chain_selectors[index] == target_chain_selector {
                self.token_ids.remove(index);
                self.token_addresses.remove(index);
                self.target_balances.remove(index);
            }
        } else {
            return Err(BridgeErrorCode::UnsupportedToken.into()); // Token ID not found
        }

        Ok(())

    }

    // Get Token Address
    pub fn get_token_address(&self, token_id: u16, target_chain_selector: u32) -> Option<&Pubkey> {
        self.token_ids.iter()
            .zip(self.target_chain_selectors.iter())
            .position(|(&id, &selector)| id == token_id && selector == target_chain_selector)
            .map(|index| &self.token_addresses[index])
    }

    // Get Target Balance
    pub fn get_target_balance(&self, token_id: u16, target_chain_selector: u32) -> Result<u64> {
        if let Some(index) = self.token_ids.iter()
            .zip(self.target_chain_selectors.iter())
            .position(|(&id, &selector)| id == token_id && selector == target_chain_selector) {
            Ok(self.target_balances[index])
        } else {
            Err(BridgeErrorCode::UnsupportedToken.into()) // Token ID or Chain Selector not found
        }
    }

    // Update Balance
    pub fn update_balance(&mut self, token_id: u16, target_chain_selector: u32, amount: u64, flag: bool) -> Result<()> {
        if let Some(index) = self.token_ids.iter()
            .zip(self.target_chain_selectors.iter())
            .position(|(&id, &selector)| id == token_id && selector == target_chain_selector) {
            if flag {
                self.target_balances[index] += amount;
            } else {
                self.target_balances[index] -= amount;
            }
            Ok(())
        } else {
            Err(BridgeErrorCode::UnsupportedToken.into()) // Token ID or Chain Selector not found
        }
    }
}