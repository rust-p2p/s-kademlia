//! notes:
//! keeping ids separate from keys
//! because I will use keys for signatures
//! - hash the public key for node_id
// --> but place the puzzle here (need to benchmark this asap)


// ask to self
// -- could the generator pattern be useful here?
// --> see recent NES article...

// generic node_id
//
// - consider separating into own file
pub trait NodeId: Hash + PartialEq + Eq + Clone + Send + Sync + Debug {
    fn is_zero(&self) -> bool;
    fn distance(&self: other: &Self) -> Self;
    fn generate(bit_size: usize) -> Self;
}

impl NodeId for &u64 {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn distance(&self, other: &u64) -> u64 {
        // ...bitwise xor for kademlia
        self ^ other
    }

    fn generate(bit_size: usize) -> u64 {
        // TODO: PoW node_id generation for skademlia
        // use disco_hash for the hash function
        // -- need to benchmark to verify some difficulty
        // --> also need to check for valid number of generations
        // current is just random generation from the os
        
        // use os_rand here...
    }
}