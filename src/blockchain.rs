use super::hashable::Hashable;
use super::block::Block;
use super::Hash;
use super::check_difficulty;
use std::collections::HashSet;

/// Poor man's version of error type
#[derive(Debug)]    // when you print it out in debug format, it will print litterally the enum
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
}

impl Blockchain{
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }
    pub fn update_with_block(&mut self, block: Block)-> Result<(), BlockValidationErr>{
        let i = self.blocks.len(); // last block we are adding to the block chain 

        for (i,block) in self.blocks.iter().enumerate() {
            if block.index != i as u32 {
                return Err(BlockValidationErr::MismatchedIndex); 
            }else if !check_difficulty(&block.hash(),block.difficulty){
                return Err(BlockValidationErr::InvalidHash);
            }else if i != 0 {
                // not genesis block , verify time increases
                let prev_block = &self.blocks[i-1];
                if block.timestamp <= prev_block.timestamp {
                    return Err(BlockValidationErr::AchronologicalTimestamp);
                }else if block.prev_block_hash != prev_block.hash {
                    return Err(BlockValidationErr::MismatchPreviousHash);
                }
            }else{
                // genesis block
                if block.prev_block_hash != vec![0;32]{
                    return Err(BlockValidationErr::InvalidGenesisBlockFormat);
                }
            }
        }
        
        // if let : allows you to destructure a value conditionally, Some returns some or none
        if let Some((coinbase,transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }
            let mut block_spent:HashSet<Hash> = HashSet::new();
            let mut block_created:HashSet<Hash> = HashSet::new();

            let mut total_fee = 0;
            for transaction in transactions {
                let input_hashes = transaction.input_hashes(); 
                if !(&input_hashes - &self.unspent_outputs).is_empty() //not going to prevent you from using same output as input twice as block
                    || !(&input_hashes & &block_spent).is_empty() // This checks to see if you are not using same output as input twice
                    { 
                    return Err(BlockValidationErr::InvalidInput);
                }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();
                if output_value> input_value{
                    return Err(BlockValidationErr::InsufficientInputValue);
                }
                let fee = input_value-output_value;
                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }
            if coinbase.output_value() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }else{
                block_created.extend(coinbase.output_hashes());
            }

            // Update the pool
            self.unspent_outputs.retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
            self.blocks.push(block); // add block to blockchain
        }
        
        return Ok(());
    }
}