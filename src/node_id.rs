use crate::ed25519::{Keypair, PublicKey};
use crate::error::{NodeIdGenError, ParseError};
use crate::node::NodeInfo;
use bs58;
use disco::{hash, DiscoHash};
use std::{
    cmp::Ordering,
    convert::{TryFrom, TryInto},
    fmt,
    str::FromStr,
};
use time::{Duration, SteadyTime};

/// NodeId
///
/// fn generate() { disco::hash(public_key) }
#[derive(Clone, Eq)]
pub struct NodeId {
    pub discohash: Vec<u8>,
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NodeId").field(&self.to_base58()).finish()
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_base58().fmt(f)
    }
}

impl NodeId {
    /// Builds a `NodeId` from a public key.
    #[inline]
    pub fn from_public_key(key: PublicKey) -> Result<NodeId, NodeIdGenError> {
        let keyhash = hash(key.as_bytes(), 32);
        let new_node = NodeId { discohash: keyhash };
        if new_node.is_zero() {
            return Err(NodeIdGenError::PubkeyHashZero);
        } else {
            return Ok(new_node);
        }
    }

    /// Generate NodeId
    ///
    /// Default no hard mechanism for slowing id generation
    #[inline]
    pub fn generate() -> Result<NodeId, NodeIdGenError> {
        // TODO: private key must be stored somewhere for signing messages?
        let key = Keypair::generate(&mut rand::thread_rng());
        let keyhash = hash(key.public.as_bytes(), 32);
        let new_node = NodeId { discohash: keyhash };
        if new_node.is_zero() {
            return Err(NodeIdGenError::PubkeyHashZero);
        } else {
            return Ok(new_node);
        }
    }

    /// Generate NodeId with Resistance
    ///
    /// Requires disco::hash(public_key) to be have `difficulty` number of trailing zeros
    /// TODO: hash should be of some changing shared state for better sybil resistance
    pub fn hard_generate(difficulty: usize, timeout: usize) -> Result<NodeId, NodeIdGenError> {
        let clock = SteadyTime::now();
        loop {
            let new_id = NodeId::generate()?;
            // default trailing zeros (remove `rev()` for leading zeros)
            //let disco_iter = new_id.discohash.iter().rev();
            let mut success = true;
            for i in 0..difficulty {
                if new_id.discohash.get(i).unwrap() != &0u8 {
                    success = false;
                }
            }
            if success {
                return Ok(new_id);
            }
            //                           converts usize into i64 and panics if doesn't fit
            // TODO: consider if this is best or if I should just pass i64 as an argument
            if SteadyTime::now() - clock > Duration::seconds(timeout.try_into().unwrap()) {
                return Err(NodeIdGenError::HardGenTimeOut);
            }
        }
    }

    #[inline]
    pub fn to_base58(&self) -> String {
        bs58::encode(self.discohash.as_slice()).into_string()
    }

    #[inline]
    fn digest(&self) -> &[u8] {
        &self.discohash.as_slice()
    }

    // TODO: better verification that `data` is a valid `NodeId`
    // would require logic from `is_public_key`
    #[inline]
    fn from_bytes(data: Vec<u8>) -> Result<NodeId, ParseError> {
        if data.len() != 32 {
            return Err(ParseError);
        }
        let new_node = NodeId { discohash: data };
        Ok(new_node)
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
    /// Note: does not generate keypair, just generates a random byte array
    #[inline]
    pub fn random(output_len: usize) -> NodeId {
        NodeId {
            discohash: DiscoHash::random(output_len),
        }
    }

    /// Useful for distance metric and node_id generation checks
    pub fn is_zero(&self) -> bool {
        self.discohash.iter().all(|d| *d == 0)
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
}

impl FromStr for NodeId {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = bs58::decode(s).into_vec()?;
        NodeId::from_bytes(bytes).map_err(|_| ParseError)
    }
}

impl Ord for NodeId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.discohash.cmp(&other.discohash)
    }
}

impl PartialOrd for NodeId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<PublicKey> for NodeId {
    #[inline]
    fn from(pubkey: PublicKey) -> NodeId {
        let keyhash = hash(pubkey.as_bytes(), 32);
        NodeId { discohash: keyhash }
    }
}

impl PartialEq for NodeId {
    fn eq(&self, other: &NodeId) -> bool {
        self.discohash == other.discohash
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

pub trait KadMetric: PartialEq + Clone + fmt::Debug {
    fn distance(&self, other: &Self) -> Self;
}

impl KadMetric for NodeId {
    fn distance(&self, other: &NodeId) -> NodeId {
        let dist = self
            .discohash
            .iter()
            .zip(other.discohash.iter())
            .map(|(first, second)| first ^ second)
            .collect();
        NodeId { discohash: dist }
    }
}

// TODO: consider impls for...
// AsRef<[u8]> for NodeId
// impl FromStr for NodeId

#[cfg(test)]
mod tests {
    use super::{KadMetric, NodeId};
    use crate::ed25519::Keypair;
    use crate::error::ParseError;
    use disco::hash;
    use rand;

    #[test]
    fn node_id_is_public_key() {
        let key = Keypair::generate(&mut rand::thread_rng());
        let keyhash = hash(key.public.as_bytes(), 32);
        let node_id = NodeId { discohash: keyhash };
        assert!(node_id.is_public_key(key.public));
    }

    #[test]
    fn random_node_id_is_valid() {
        for _ in 0..5000 {
            let node_id = NodeId::random(32);
            let test_node_id = NodeId {
                discohash: node_id.discohash.clone(),
            };
            assert_eq!(node_id, test_node_id);
        }
    }

    #[test]
    fn distance_from_self_is_zero() {
        let node_id = NodeId::generate().unwrap();
        let clone_node_id = node_id.clone();
        let distance = &node_id.distance(&clone_node_id);
        assert!(distance.is_zero());
    }

    #[test]
    fn to_base58_then_back() {
        let node_id = NodeId::generate().unwrap();
        let second: NodeId = node_id.to_base58().parse().unwrap();
        assert_eq!(node_id, second);
    }

    // TODO: change when error becomes more descriptive
    // for the moment, all ParseError == ParseError
    impl PartialEq for ParseError {
        fn eq(&self, other: &ParseError) -> bool {
            true
        }
    }

    #[test]
    fn incorrect_length_yields_parse_error() {
        let data = Vec::with_capacity(33);
        let data2 = Vec::with_capacity(31);
        assert_eq!(NodeId::from_bytes(data).unwrap_err(), ParseError);
        assert_eq!(NodeId::from_bytes(data2).unwrap_err(), ParseError);
    }
}
