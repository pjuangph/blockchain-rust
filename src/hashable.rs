// This is similar to a c# interface

pub trait Hashable {
    /// This is kind of like an interface
    fn bytes (&self) -> Vec<u8>;

    /// This is a function for the block. 
    /// Block's bytes which consists of all properties of the block including previous data (nonce) is converted to SHA256 hash. 
    fn hash (&self) -> Vec<u8> {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}
