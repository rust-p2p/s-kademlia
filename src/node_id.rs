use crate::ed25519::{Keypair, PublicKey, Signature};
use crate::error::{DistanceIsZero, ParseError};
use crate::node::NodeInfo;
use bs58;
use disco::{hash, DiscoHash};
use std::{
    cmp::Ordering,
    convert::{TryFrom, TryInto},
    fmt,
    str::FromStr,
};
use uint::*;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

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
    pub fn from_public_key(key: PublicKey) -> NodeId {
        let keyhash = hash(key.as_bytes(), 32);
        NodeId { discohash: keyhash }
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
        let pk_hash = hash(pubkey.as_bytes(), 32);
        &pk_hash == &self.discohash
    }

    /// Useful for distance metric and node_id generation checks
    pub fn is_zero(&self) -> bool {
        self.discohash.iter().all(|d| *d == 0)
    }

    /// Verify Signature
    ///
    /// verify message signature made with the public key associated with this NodeId
    pub fn verify(pubkey: PublicKey, msg: &[u8], sig: &[u8]) -> bool {
        Signature::from_bytes(sig).and_then(|s| pubkey.verify(msg, &s)).is_ok()
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
    type Err;
    type Metric;

    // used in `store` specifically
    fn distance(&self, other: &Self) -> Result<Self::Metric, Self::Err>;
}

impl KadMetric for NodeId {
    type Err = DistanceIsZero;
    type Metric = U256;

    fn distance(&self, other: &NodeId) -> Result<U256, DistanceIsZero> {
        let a = U256::from(self.discohash.clone().as_slice());
        let b = U256::from(other.discohash.clone().as_slice());
        // xor
        let distance = a ^ b;
        if distance == U256::from(0) {
            return Err(DistanceIsZero);
        } else {
            return Ok(distance);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{KadMetric, NodeId};
    use crate::ed25519::{Keypair, PublicKey, Signature};
    use crate::error::{DistanceIsZero, ParseError};
    use disco::hash;
    use rand;

    // TODO: change when error becomes more descriptive
    // for the moment, all ParseError == ParseError
    impl PartialEq for ParseError {
        fn eq(&self, other: &ParseError) -> bool {
            true
        }
    }

    // \A DistanceIsZero == DistanceIsZero
    impl PartialEq for DistanceIsZero {
        fn eq(&self, other: &DistanceIsZero) -> bool {
            true
        }
    }

    #[test]
    fn node_id_is_public_key() {
        let key = Keypair::generate(&mut rand::thread_rng());
        let node_id = NodeId::from_public_key(key.public);
        assert!(node_id.is_public_key(key.public));
    }

    #[test]
    fn sign_and_verify_works() {
        let kp = Keypair::generate(&mut rand::thread_rng());
        let node_id = NodeId::from_public_key(kp.public);
        let msg = vec![1u8; 32];
        let msg_copy = msg.clone();
        let sig = kp.sign(&msg.as_slice()).to_bytes();
        assert!(NodeId::verify(kp.public, msg_copy.as_slice(), &sig));
    }

    #[test]
    fn distance_works() {
        let key = Keypair::generate(&mut rand::thread_rng());
        let node_id = NodeId::from_public_key(key.public);
        let clone_node_id = node_id.clone();
        let distance = &node_id.distance(&clone_node_id);
        // distance from other key
        let new_key = Keypair::generate(&mut rand::thread_rng());
        let new_node_id = NodeId::from_public_key(new_key.public);
        let distance2 = &node_id.distance(&new_node_id);
        assert_eq!(distance.as_ref().unwrap_err(), &DistanceIsZero);
        // assert!(distance2.as_ref().unwrap() != &DistanceIsZero); // if uncommented, compiler error below generated `=>` ok I guess
        //                                      ^^ no implementation for `node_id::NodeId == error::DistanceIsZero`
    }

    #[test]
    fn to_base58_then_back() {
        let key = Keypair::generate(&mut rand::thread_rng());
        let node_id = NodeId::from_public_key(key.public);
        let second: NodeId = node_id.to_base58().parse().unwrap();
        assert_eq!(node_id, second);
    }

    #[test]
    fn incorrect_length_yields_parse_error() {
        let big_data = vec![1u8; 33];
        let little_data = vec![1u8; 31];
        let ok_data = vec![1u8; 32];
        assert_eq!(NodeId::from_bytes(big_data).unwrap_err(), ParseError);
        assert_eq!(NodeId::from_bytes(little_data).unwrap_err(), ParseError);
        assert!(NodeId::from_bytes(ok_data).is_ok());
    }
}
