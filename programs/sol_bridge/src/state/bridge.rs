use anchor_lang::prelude::*;
use tiny_keccak::{Hasher, Keccak};
use hex::encode;
use std::cmp::Ordering;

use crate::error::BridgeErrorCode;

#[account]
#[derive(Default)]
pub struct Bridge {
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub protocol_fee: u64,
    pub chain_selector: u64,
    pub token_ids: Vec<String>,
    pub token_addresses: Vec<Pubkey>,
    pub target_token_addresses: Vec<String>,
    pub target_balances: Vec<u64>,
    pub target_chain_selectors: Vec<u64>,
}

impl Bridge {
    // Helper function to perform keccak256 hashing
    fn keccak256(data: &[u8]) -> String {
        let mut hasher = Keccak::v256();
        let mut output = [0u8; 32];
        hasher.update(data);
        hasher.finalize(&mut output);
        // Convert the output (byte array) to a hex string
        encode(output)
    }

    pub fn get_token_id(
        &self,
        local_token: &[u8],        // Local token address (on Solana)
        chain_selector: u64,       // Solana chain selector (uint64)
        remote_chain_selector: u64,// EVM chain selector (uint64)
        remote_token: &[u8],       // Remote token address (on EVM)
    ) -> Result<String> {          // Token ID (hex string)
    
        // Compare keccak256 of local_token and remote_token
        let local_hash = Self::keccak256(local_token);
        let remote_hash = Self::keccak256(remote_token);
    
        // Determine the order based on keccak256 hashes
        let token_id_input = match local_hash.cmp(&remote_hash) {
            Ordering::Less => {
                // If localToken hash is smaller, order: chainSelector, localToken, remoteChainSelector, remoteToken
                [
                    chain_selector.to_le_bytes().to_vec(), // Convert to Vec<u8>
                    local_token.to_vec(),
                    remote_chain_selector.to_le_bytes().to_vec(),
                    remote_token.to_vec()
                ].concat()
            }
            _ => {
                // Otherwise, order: remoteChainSelector, remoteToken, chainSelector, localToken
                [
                    remote_chain_selector.to_le_bytes().to_vec(),
                    remote_token.to_vec(),
                    chain_selector.to_le_bytes().to_vec(),
                    local_token.to_vec()
                ].concat()
            }
        };
    
        // Compute the keccak256 of the concatenated result
        let token_id_hash = Self::keccak256(&token_id_input);
    
        // Convert the keccak256 hash (likely a byte array) to a hex string
        Ok(hex::encode(token_id_hash)) // `hex` crate for hex encoding
    }
    
    pub fn add_token(
        &mut self,
        local_token: Pubkey,        // Local token address (on Solana)
        remote_chain_selector: u64, // EVM chain selector (uint64)
        remote_token: String,       // Remote token address (on EVM)
    ) -> Result<String> {
        // Encode local_token as bytes
        let binding = local_token.to_string();
        let local_token_bytes = binding.as_bytes();

        // Compute the token ID using get_token_id function
        let token_id = Self::get_token_id(
            &self,
            local_token_bytes,       // Local token as bytes
            self.chain_selector,      // Solana chain selector (from Bridge struct)
            remote_chain_selector,    // EVM chain selector
            remote_token.as_bytes()   // Remote token as bytes
        )?;

        // Check if token ID already exists in the token_ids vector
        if let Some(index) = self.token_ids.iter().position(|id| id == &token_id) {
            // Check if the chain selector matches
            if self.target_chain_selectors[index] == remote_chain_selector {
                return Err(BridgeErrorCode::AlreadyExist.into()); // Token already registered
            }
        }

        // Add the token if it doesn't already exist
        self.token_ids.push(token_id.clone());
        self.token_addresses.push(local_token);
        self.target_token_addresses.push(remote_token);
        self.target_balances.push(0); // Initialize balance with 0
        self.target_chain_selectors.push(remote_chain_selector); // Store the chain selector
        
        Ok(token_id)
    }

    pub fn remove_token(
        &mut self,
        local_token: Pubkey,        // Local token address (on Solana)
        remote_chain_selector: u64, // EVM chain selector
        remote_token: String,       // Remote token address (on EVM)
    ) -> Result<String> {
       // Encode local_token as bytes
       let binding = local_token.to_string();
       let local_token_bytes = binding.as_bytes();


        // Compute the token ID using get_token_id function
        let token_id = Self::get_token_id(
            &self,
            local_token_bytes,       
            self.chain_selector,      // Solana chain selector
            remote_chain_selector,    
            remote_token.as_bytes()   // Remote token as bytes
        )?;

        // Find the index of the token ID in token_ids
        if let Some(index) = self.token_ids.iter().position(|id| id == &token_id) {
            if self.target_chain_selectors[index] == remote_chain_selector {
                // Remove token info if found
                self.token_ids.remove(index);
                self.token_addresses.remove(index);
                self.target_balances.remove(index);
                self.target_token_addresses.remove(index);
                self.target_chain_selectors.remove(index);
                Ok(token_id)
            } else {
                Err(BridgeErrorCode::UnsupportedToken.into()) // Chain selector mismatch
            }
        } else {
            Err(BridgeErrorCode::UnsupportedToken.into()) // Token ID not found
        }
    }

    // Get Token Address
    pub fn get_token_address(
        &self,
        token_id: String,   
    ) -> Option<&Pubkey> {
        // Find the index of the token ID in token_ids
        self.token_ids.iter()
            .position(|id| id == &token_id)
            .map(|index| &self.token_addresses[index])
    }

}