use blockchainlib::{Block,Blockchain,now, Transaction,transaction};

// Note about Mining: You cannot derive the data from the hash but you can get the hash from whatever data

//Difficulty - the unsigned 128 bit integer value that the most significant 16 bytes of hash must be less than before it's considered valid. Number of bits/bytes at beginning of the hash that must be 0 

fn main () {
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
        Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output{
                    value:50,
                    to_addr:"Alice".to_owned()
                },
                transaction::Output{
                    value:7,
                    to_addr:"Bob".to_owned()
                },
            ]
        }
    ], difficulty);
    genesis_block.mine();
    println!("Mined genesis block {:?}", &genesis_block);
    
    let mut last_hash = genesis_block.hash.clone();
    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block).expect("failed to add genesis block");

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

