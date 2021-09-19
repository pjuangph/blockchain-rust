use blockchainlib::{Block,Hashable,Blockchain,now};

// Note about Mining: You cannot derive the data from the hash but you can get the hash from whatever data

//Difficulty - the unsigned 128 bit integer value that the most significant 16 bytes of hash must be less than before it's considered valid. Number of bits/bytes at beginning of the hash that must be 0 

fn main () {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;
    let mut block = Block::new(0, now(), vec![0; 32], 0, "Genesis block!".to_owned(), difficulty);
    block.mine();
    println!("Mined genesis block {:?}", &block);

    let mut last_hash = block.hash.clone();
    let mut blockchain = Blockchain {
        blocks: vec![block],
    };
    

    for i in 1..=10 { // loop 1 to 10 
        let mut block = Block::new(i, now(), last_hash, 0, "Another block".to_owned(), difficulty);

        block.mine(); 
        println!("Mined block {:?}", &block);
        last_hash = block.hash.clone();
        blockchain.blocks.push(block);
        println!("Verify: {}", &blockchain.verify());
    }
    blockchain.blocks[3].index = 4; 
    println!("Verify: {}", &blockchain.verify());
}

// Block verification
// 1. Actual index == stored index value 
// 2. Block's hash fits stored difficulty value (it's insecure to simply trust the difficulty)
// 3. Time is always increasing 
// 4. Actual previous block's hash == stored prev_block_hash value 
// 5. Blockchain is a distributed ledger, everyone has a history of where they got their coins

// Block Transactions
// Alice has 50 coins
// Bob has 7 coins
// Alice sends bob coins
// * A Blockchain isn't a spreadsheet!
// Needs:
//  - Protect against overspending
//  - No double spending
//  - Impersonation (who owns the money)
//  - More but need to look into https://en.bitcoin.it/wiki/Protocol_rules#.22tx.22_messages 

