use blockchainlib::{Block,Hashable};

// Note about Mining: You cannot derive the data from the hash but you can get the hash from whatever data

//Difficulty - the unsigned 128 bit integer value that the most significant 16 bytes of hash must be less than before it's considered valid. Number of bits/bytes at beginning of the hash that must be 0 

// Little vs. Big Endian - The order of bytes stored in memory
// Example 42u32
// Big Endian 00 00 00 2a
// Little Endian 2a 00 00 00
fn main () {
    // difficulty = 0x00 first 16 bytes are 15
    let difficulty = 0x0000ffffffffffffffffffffffffffff;
    let mut block = Block::new(0, 0, vec![0; 32], 0, "Genesis block!".to_owned(), difficulty);

    block.hash = block.hash();

    println!("{:?}", &block);

    block.mine(); 

    println!("{:?}", &block);
}
