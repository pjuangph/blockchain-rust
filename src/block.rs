use std::fmt::{ self, Debug, Formatter };
use super::{BlockHash,u128_bytes, u32_bytes, u64_bytes, Hashable};

/// Strategy 
/// 1. Generate a nonce
/// 2. Hash bytes (heavy step)
/// 3. Check hash against difficulty 
/// 4. Add Block to chain
/// 5. Submit to peers (skip this step)

pub struct Block {
    /// nonce - if you generate a hash and doesn't fit the difficulty so we change bytes that we are hashing but this is bad. So we introduce nonce. Nonce is arbitrary data and change it at will and eventually when hashed, that will generate a hash that matches the difficulty. 
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64, 
    pub payload: String,
}

impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.payload,
        )
    }
}

impl Block {
    pub fn new (index: u32, timestamp: u128, prev_block_hash: BlockHash, nonce: u64, payload: String) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce,
            payload,
        }
    }
}

impl Hashable for Block {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.payload.as_bytes());

        bytes
    }
}
