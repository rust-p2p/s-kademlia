//! NodeId
//! NodeId is a type with a marker-based implementation
//! - to allow for future proofing in the context of content-addressed
//! identifiers...? (ie multihash)
use disco::DiscoHash;
use std::{convert::TryFrom, fmt, str::FromStr};
use crate::error::AntiSybilError;
use crate::key::KeyPair;
use multihash::MultiHash;

/// NodeId
///
/// trait for `Node` in ./node.rs
pub trait NodeId: MultiHash + PartialEq + Eq + Clone + Send + Sync + Debug {
    type Metric: PartialEq + Eq + Clone; // add symmetricity for iterative, parallel lookups (anti-eclipse measure)

    fn generate_from_pubkey(pubkey: PublicKey) -> Self;
    fn distance(&self: other: &Self) -> Self::Metric;
}

// separate trait (layered logic, modularity)
// could just use `Into` and `From`
// methods for from_key()
// and into_key()

// id should be implemented by default for multihash

impl NodeId for multihash::Hash {
    type Metric: multihash::Hash;

    fn generate_from_pubkey(pubkey: PublicKey) -> Result<Self, AntiSybilError> {
        // TODO: must ensure that the pubkey input was generated with `crate::key::KeyPair::new()`
        // or the anti-sybil mechanism is dumb (as it already is...)
        let key = KeyPair::new()?;
        // issue #3: 
    }
}

impl NodeId for &u64 {
    type Metric: &u64; // feels unnecessary?

    fn generate() -> Result<u64, AntiSybilError> {
        let key = KeyPair::new()?;
        // TODO: this unwrap is safe because of ? above...?
        let public_key = key.unwrap().public();

        // hash the public key to generate a `NodeId`
        // -- ongoing concern is regarding where to store `NodeId` (<)=(>) `PublicKey` 
        // to minimize indirection and duplicate mappings (authentication will pertain to this)
    }

    fn distance(&self, other: &u64) -> u64 {
        // default bitwise xor (should probably apply the metric here somehow)
        // - symmetricity fosters parallel, disjoint lookups
        self ^ other
    }

    #[inline]
    pub fn from_public_key(key: PublicKey) -> NodeId {
        let key_enc = key.into_protobuf_encoding();
        // TODO: note rust-libp2p incompatibility cited here and make choice
        let hash_algorithm = disco::DiscoHash;
        let multihash = 
    }
}

impl NodeId for Vec<u8> {

}