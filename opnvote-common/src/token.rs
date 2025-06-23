/*
opnVote - Reliable, verifiable and secure electronic voting systems
Copyright (C)  2025  Max Bossing <max@bossi.ng>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use std::fs;
use base64::Engine;
use ring::{signature};
use ring::rand::SystemRandom;
use ring::signature::{Ed25519KeyPair, KeyPair, ED25519};
use serde::{Deserialize, Serialize};
use crate::token::TokenError::AlreadyRevoked;

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishToken {
    pub key: String,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteToken {
    pub pk: String,
    pub sk: String,
    pub signature: String,
}

#[derive(Debug)]
pub enum TokenError {
    Ok(),
    AlreadyRevoked(),
    InvalidSignature(),
    InvalidKey(String),
    PrototypingError(String),
}

impl VoteToken {
    
    pub fn to_publish_token(&self) -> PublishToken {
        PublishToken { key: self.pk.clone(), signature: self.signature.clone() }
    }
    
    pub fn new(class_private_key_path: &str) -> Result<Self, TokenError> {
        let pkcs8 = Ed25519KeyPair::generate_pkcs8(&SystemRandom::new()).unwrap();
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).unwrap();
        let private_key = base64::prelude::BASE64_URL_SAFE.encode(pkcs8.as_ref());
        let pub_key = key_pair.public_key();
        let pub_key = base64::prelude::BASE64_URL_SAFE.encode(pub_key.as_ref());
        
        // TODO: this is mvp-code, keys are read from a smart card
        let parent_key_bytes = fs::read(class_private_key_path).map_err(|_| TokenError::InvalidKey("Missing key file".to_string()))?;
        let parent_key = signature::Ed25519KeyPair::from_pkcs8(&parent_key_bytes).map_err(|_| TokenError::InvalidKey("Missing key file".to_string()))?;
        let parent_public_key = parent_key.public_key().as_ref();
        let parent_public_key = signature::UnparsedPublicKey::new(&signature::ED25519, parent_public_key);
        
        let signature = parent_key.sign(pub_key.as_bytes());
        
        // Check that signature is correct
        parent_public_key.verify(pub_key.as_ref(), signature.as_ref()).map_err(|_| TokenError::InvalidSignature())?;
        
        let signature = base64::prelude::BASE64_URL_SAFE.encode(signature.as_ref());
        
        Ok(Self { pk: pub_key, sk: private_key, signature })
    }
    
    pub fn verify_signature(&self, class_public_key_path: &str) -> Result<(), TokenError> {
        let public_key_bytes = fs::read(class_public_key_path).map_err(|_| TokenError::InvalidKey("Missing key file".to_string()))?;
        let public_key = signature::UnparsedPublicKey::new(&ED25519, public_key_bytes);
        let signature = base64::prelude::BASE64_URL_SAFE.decode(self.signature.as_str()).map_err(|_| TokenError::InvalidSignature())?;
        public_key.verify(self.pk.as_bytes(), signature.as_ref()).map_err(|_| TokenError::InvalidSignature())?;
        Ok(())
    }
    
    pub fn revoke(&self) -> Result<(), TokenError> {
        //TODO: this is all mvp-fake-code, revoked keys are handled by the vote controller
        //TODO: Check if already revoked bruv
        let mut revoked_list = fs::read_to_string("resources/revoked_tokens.txt").unwrap_or_default();
        revoked_list.push_str(&format!("{}\n", self.pk));
        let _ = fs::write("resources/revoked_tokens.txt", revoked_list).unwrap();
        Ok(())
    }
    
    pub fn is_revoked(&self) -> bool {
        if let Ok(content) = fs::read_to_string("resources/revoked_tokens.txt") {
            return content.lines().any(|line| line.trim() == self.pk);
        }
        false
    }

    pub fn sign_vote_and_revoke(&self, vote: i32) -> Result<String, TokenError> {
        if self.is_revoked() { Err(AlreadyRevoked())? }; 
        let key_bytes = base64::prelude::BASE64_URL_SAFE.decode(self.sk.as_str()).map_err(|_| TokenError::InvalidKey("Invalid Private key!".to_string()))?;
        let key = Ed25519KeyPair::from_pkcs8(&key_bytes).unwrap();
        let signature = key.sign(vote.to_string().as_bytes());
        self.revoke()?;
        Ok(base64::prelude::BASE64_URL_SAFE.encode(signature.as_ref()))
    }
}