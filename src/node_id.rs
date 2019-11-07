use bs58;
use disco::{hash, DiscoHash};
use time::{Duration, SteadyTime};
use std::{convert::{TryFrom, TryInto}, fmt, str::FromStr};
use crate::ed25519::{Keypair, PublicKey};
// use crate::error::TimeOutError;
use crate::node::NodeInfo;

/// NodeId
///
/// - contains a vector of bytes
/// fn generate() { disco::hash(public_key) }
#[derive(Clone, Eq, PartialEq)]
pub struct NodeId {
    pub discohash: Vec<u8>,
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NodeId")
            .field(&self.to_base58())
            .finish()
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_base58().fmt(f)
    }
}

impl NodeId {
    /// Generate NodeId
    ///
    /// Default no hard mechanism for slowing id generation
    #[inline]
    pub fn generate() -> NodeId {
        // TODO: private key must be stored somewhere for signing messages?
        let key = Keypair::generate(&mut rand::thread_rng());
        let keyhash = hash(key.public.as_bytes(), 32);
        return NodeId { discohash: keyhash }
    }
    
    /// Generate NodeId with Resistance
    ///
    /// Requires disco::hash(public_key) to be have `difficulty` number of trailing zeros
    pub fn hard_generate(difficulty: usize, timeout: usize) -> NodeId {
        let clock = SteadyTime::now();
        loop {
            // TODO: replace with generation of PublicKey to hash and then choose an ID
            // - could be more generic, requiring some configuration
            let new_id = NodeId::generate();
            // default trailing zeros (remove `rev()` for leading zeros)
            //let disco_iter = new_id.discohash.iter().rev();
            let mut success = true;
            for i in 0..difficulty {
                if new_id.discohash.get(i).unwrap() != &0u8 {
                    success = false;
                }
            }
            if success {
                return new_id
            }
            //                           converts usize into i64 and panics if doesn't fit
            // TODO: consider if this is best or if I should just pass i64 as an argument
            if SteadyTime::now() - clock > Duration::seconds(timeout.try_into().unwrap()) {
                // TODO: add error type
                panic!()
            }
        }
    }

    #[inline]
    pub fn to_base58(&self) -> String {
        bs58::encode(self.discohash.as_slice()).into_string()
    } // TODO: add decoding/encoding error

    #[inline]
    fn digest(&self) -> &[u8] {
        &self.discohash.as_slice()
    }

    #[inline]
    fn is_public_key(&self, pubkey: PublicKey) -> bool {
        let ret_val = true;
        let mut counter = 0;
        let pk_hash = hash(pubkey.as_bytes(), 32);
        &pk_hash == &self.discohash
    }

    /// For Testing Purposes Only
    ///
    /// note: does not generate keypair,
    /// just generates a random byte array
    #[inline]
    pub fn random(output_len: usize) -> NodeId {
        NodeId {
            discohash: DiscoHash::random(output_len),
        }
    }

    /// Verify Signature
    ///
    /// verify message signature made with the public key associated with this NodeId
    pub fn verify_msg(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
        // change receive type to an error on verifying signatures

        // would need to use the sign method in Node
        // or take a signature
        todo!();
    }

    /// Returns a raw bytes representation
    ///
    /// Prefer iteration over this in lieu of discohash
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.discohash.as_slice()
    }
}

impl From<PublicKey> for NodeId {
    #[inline]
    fn from(pubkey: PublicKey) -> NodeId {
        let keyhash = hash(pubkey.as_bytes(), 32);
        NodeId { discohash: keyhash }
    }
}

impl PartialEq<NodeInfo> for NodeId {
    fn eq(&self, other: &NodeInfo) -> bool {
        *self == other.id
    }
}

impl PartialEq<Vec<u8>> for NodeId {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.discohash == *other
    }
}

pub trait KadMetric: PartialEq + Eq + Ord + Clone + Send + Sync + fmt::Debug {
    fn distance(&self, other: &Self) -> Self;
    fn is_zero(&self) -> bool;
    fn bits(&self) -> usize;
} // could also add or separate encoding/decoding

impl KadMetric for Vec<u8> {
    fn distance(&self, other: &Vec<u8>) -> Vec<u8> {
        // TODO: check if can use `into_iter` or if it helps
        self.iter()
            .zip(other.iter())
            .map(|(first, second)| first ^ second)
            .collect()
    }

    fn is_zero(&self) -> bool {
        self.iter().all(|d| *d == 0)
    }

    // for store::NodeTable::bucket_number() based on distance()
    // TODO: where is this found and what can I do with it...
    // -- replace with byteorder
    // fn bits(&self) -> usize {
    //     let mut bits = self.len() * 8;
    //     self.into_iter().for_each(|b| {
    //         if *b == 0 {
    //             bits -= 8;
    //         } else {
    //             bits -= (b.leading_zeros() as usize);
    //         }
    //     });
    //     assert!(bits == 0);
    //     0
    // }
    fn bits(&self) -> usize {
        todo!()
    }
}

// TODO: consider impls for...
// AsRef<[u8]> for NodeId
// impl FromStr for NodeId

#[cfg(test)]
mod tests {
    use super::{NodeId, KeyPair, PublicKey};
    use crate::rand;

    #[test]
    fn node_id_is_public_key() {
        let key = Keypair::generate(&mut rand::thread_rng());
        let node_id = key.public.clone().into();
        assert_eq!(node_id.is_public_key(&key.public), Some(true));
    }

    #[test]
    fn node_id_to_base58_then_back() {
        let node_id = NodeId::generate();
        let second: NodeId = node_id.to_base58().parse().unwrap();
        assert_eq!(node_id, second);
    }

    #[test]
    fn random_node_id_is_valid() {
        for _ in 0 .. 5000 {
            let node_id = NodeId::random(32);
            assert_eq!(node_id, NodeId { node_id.discohash.clone() });
        }
    }
}
