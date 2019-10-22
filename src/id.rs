use crate::ed25519;
use std::{convert::TryFrom, fmt, str::FromStr};
use disco::DiscoHash;
use uint::*; // U256

/// NodeId Struct
///
/// - wrapper around DiscoHash
/// - default implements the trait described below 
/// - easy TODO: refactor to make more generic (don't specify DiscoHash)
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NodeId {
    pub id: DiscoHash,
}

/// The metric for DHT NodeIds
///
/// - separate the ID from the topology logic
#[derive(Copy, Clone, PartialEq, Eq, Default, PartialOrd, Ord, Debug)]
pub struct Distance(pub(super) U256);

impl NodeId {
    /// Builds a `NodeId` from a public key
    #[inline]
    fn from_public_key(key: PublicKey) -> NodeId {
        let mut h = DiscoHash::new(32);
        h.write(pubkey.as_bytes());
        NodeId { h.sum() }
    }

    /// Checks whether `data` is a valid `NodeId`. if so, returns the `NodeId`. If not,
    /// returns back the data as an error,
    #[inline]
    pub fn from_bytes(data: Vec<u8>) -> Result<NodeId,  Vec<u8>> {
        let mut h = DiscoHash::new(32)
        h.write(data.as_bytes());
        Ok(NodeId { h.sum() })
    }

    /// Turns a `DiscoHash` into a `NodeId`. Dumb simple by construction
    #[inline]
    pub fn from_discohash(data: DiscoHash) -> Result<NodeId, DiscoHash> {
        Ok(NodeId { data })
    }

    // TODO: useful to generate random NodeId function
    // - to randomly walk the DHT like R5N (make issue)

    /// TODO: add discohash.as_bytes() method to disco
    /// ---> do I also want base-58 encoded string?

    /// Default distance by XOR metric
    pub fn distance<U>(&self, other: &U) -> Distance
    where
        U: AsRef<NodeId>
    {
        let a = U256::from(self.0.as_ref());
        let b = U256::from(other.as_ref().0.as_ref());
        Distance(a ^ b)
    }
}

impl From<PublicKey> for NodeId {
    #[inline]
    fn from(key: PublicKey) -> NodeId {
        NodeId::from_public_key(key)
    }
}

impl TryFrom<Vec<u8>> for NodeId {
    type Error: Vec<u8>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        NodeId::from_bytes(value)
    }
}

impl Tryfrom<DiscoHash> for NodeId {
    type Error: DiscoHash;

    fn try_from(data: DiscoHash) -> Result<Self, Self::Error> {
        NodeId::from_discohash(data)
    }
}

// TODO:
// - PartialEq<DiscoHash> for NodeId
// - PartialEq<NodeId> for DiscoHash
// - AsRef<DiscoHash> for NodeId
// - AsRef<[u8]> for NodeId
// - Into<DiscoHash> for NodeId
// - FromStr for NodeId ?