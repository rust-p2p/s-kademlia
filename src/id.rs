//! notes:
//! keeping ids separate from keys
//! because I will use keys for signatures
//! - hash the public key for node_id
// --> but place the puzzle here (need to benchmark this asap)
#![feature(generators, generator_trait)]

use std::pin::Pin;
use std::ops::{Generator, GeneratorState};
use disco::DiscoHash;


// ask to self
// -- could the generator pattern be useful here?
// --> see recent NES article...

// generic node_id
//
// - consider separating into own file
pub trait NodeId: Hash + PartialEq + Eq + Clone + Send + Sync + Debug {
    type Metric: PartialEq + Eq + Clone;

    fn is_zero(&self) -> bool;
    fn distance(&self: other: &Self) -> Self::Metric;
    fn generate(bit_size: usize) -> Self;
}

/// # hash
//// - used to generate NodeId with a public key
//
/// Hashes an input of any length and obtain an output of length greater or equal to
/// 256 bits (32 bytes).
///
/// Panics when `output_len < 32`.
pub fn hash(input_data: &[u8], output_len: usize) -> Vec<u8> {
    let mut h = DiscoHash::new(output_len);
    h.write(input_data);
    h.sum()
}

// s-kademlia impl
impl NodeId for &u64 {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn distance(&self, other: &u64) -> u64 {
        // bitwise xor
        // - symmetricity fosters parallel, disjoint lookups
        self ^ other
    }

    // place this in the keypair generation formula
    fn generate(pub_key: PublicKey) -> u64 {
        // TODO: PoW node_id generation for skademlia
        // use disco_hash for the hash function
        // -- need to benchmark to verify some difficulty
        // --> also need to check for valid number of generations
        // current is just random generation from the os
        
        // use os_rand here...
        // -- use a generator here for generating an id
        // -- use a generator here or some loop structure to asynchronously hash until the `NodeId`
        // has a certain number of trailing 0s? (how does this make sense with u64)
        // -- end when we finally get to a value, but yield otherwise
    }
}