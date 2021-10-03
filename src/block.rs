use std::fmt::{ self, Debug, Formatter };
use super::{Hash,u128_bytes, u32_bytes, u64_bytes, Hashable, check_difficulty, Transaction};

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
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64, 
    pub transactions: Vec<Transaction>,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {} nonce: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.transactions.len(),
            &self.nonce,
        )
    }
}

impl Block {
    pub fn new (index: u32, timestamp: u128, prev_block_hash: Hash, transactions: Vec<Transaction>, difficulty: u128) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce: 0,
            transactions,
            difficulty,
        }
    }

    pub fn mine(&mut self){
        //! Loop through every single u64 bit value until we find one that where the 16 significant bytes fit the difficulty
        for nonce_attempt in 0..(u64::max_value()){
            self.nonce = nonce_attempt;
            let hash = self.hash();
            // takes the last 16 bytes and do a simple greater than comparison 
            if check_difficulty(&hash, self.difficulty){
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    /// This function takes converts the contents of the block to bytes. The bytes will be converted to a hash and then mined. 
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.transactions.iter().flat_map(|transaction| transaction.bytes()).collect::<Vec<u8>>());
        bytes.extend(&u128_bytes(&self.difficulty));
        bytes
    }
}
