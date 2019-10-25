use crate::ed25519;
use bs58;
use disco::DiscoHash;
use std::hash::Hash;
use std::{convert::TryFrom, fmt, str::FromStr};
use uint::*; // U256

/// NodeId Struct
///
/// - wrapper around DiscoHash
/// - default implements the trait described below
/// - easy TODO: refactor to make more generic (don't specify DiscoHash)
pub struct NodeId<T: Hash + Eq + PartialEq + Clone> {
    pub id: T,
}
// default DiscoHash impl below
// -> can add impl NodeId<MultiHash>

/// The metric for DHT NodeIds
///
/// - to separate the ID from the topology logic
#[derive(Copy, Clone, PartialEq, Eq, Default, PartialOrd, Ord, Debug)]
pub struct Distance(pub(super) U256);
// could add impl associated with distance for by default finding the closest k-neighbors?

impl NodeId<DiscoHash> {
    /// Builds a `NodeId` from a public key
    #[inline]
    fn from_public_key(key: PublicKey) -> NodeId<DiscoHash> {
        let mut h = DiscoHash::new(32);
        h.write(pubkey.as_bytes());
        NodeId { id: h.sum() }
    }

    /// Checks whether `data` is a valid `NodeId`. if so, returns the `NodeId`. If not,
    /// returns back the data as an error,
    #[inline]
    pub fn from_bytes(data: Vec<u8>) -> Result<NodeId<DiscoHash>, Vec<u8>> {
        let mut h = DiscoHash::new(32);
        h.write(data.as_bytes());
        Ok(NodeId { id: h.sum() })
    }

    /// Turns a `DiscoHash` into a `NodeId`. Dumb simple by construction
    #[inline]
    pub fn from_discohash(hash: DiscoHash) -> Result<NodeId<DiscoHash>, DiscoHash> {
        Ok(NodeId { id: hash })
    }

    // TODO: useful to generate random NodeId function
    // - to randomly walk the DHT like R5N (make issue)

    /// TODO: add discohash.as_bytes() method to disco
    /// ---> do I also want base-58 encoded string?

    /// Default distance by XOR metric
    pub fn distance<U>(&self, other: &U) -> Distance
    where
        U: AsRef<NodeId<DiscoHash>>,
    {
        let a = U256::from(self.0.as_ref());
        let b = U256::from(other.as_ref().0.as_ref());
        Distance(a ^ b)
    }
}

impl From<PublicKey> for NodeId<DiscoHash> {
    #[inline]
    fn from(key: PublicKey) -> NodeId {
        NodeId::from_public_key(key)
    }
}

impl TryFrom<Vec<u8>> for NodeId<DiscoHash> {
    type Error = Vec<u8>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        NodeId::from_bytes(value)
    }
}

impl Tryfrom<DiscoHash> for NodeId<DiscoHash> {
    type Error = DiscoHash;

    fn try_from(data: DiscoHash) -> Result<Self, Self::Error> {
        NodeId::from_discohash(data)
    }
}

// TODO:
// - PartialEq<NodeId<DiscoHash>> for NodeId<DiscoHash>
// - PartialEq<NodeId<DiscoHash>> for DiscoHash
// - AsRef<DiscoHash> for NodeId<DiscoHash>
// - AsRef<[u8]> for NodeId<DiscoHash>
// - Into<DiscoHash> for NodeId<DiscoHash>
// - FromStr for NodeId<DiscoHash>
