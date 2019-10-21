//! NodeId
//! NodeId is a type with a marker-based implementation
//! - to allow for future proofing in the context of content-addressed
//! identifiers...? (ie multihash)
use disco::DiscoHash;
use std::{convert::TryFrom, fmt, str::FromStr};
use crate::error::AntiSybilError;
use crate::key::KeyPair;

// generic node_id
//
// - consider separating into own file
pub trait NodeId: Hash + PartialEq + Eq + Clone + Send + Sync + Debug {
    type Metric: PartialEq + Eq + Clone; // add symmetricity for iterative, parallel lookups (anti-eclipse measure)
                                        // - open issue: increasing parallelism for searches (increases robustness)

    fn generate(&self) -> Self;
    // fn is_zero(&self) -> bool; // consider changing to: is_default() or some node_state() method...
    fn distance(&self: other: &Self) -> Self::Metric;
}

// methods for from_key()
// and into_key()

impl NodeId for &u64 {
    fn generate() -> Result<u64, AntiSybilError> {
        let key = KeyPair::new()?;
        // TODO: this unwrap is safe because of ? above...?
        let public_key = key.unwrap().public();
        // if generate returns result, return 
        // use randomness to generate a public key

        // considered using PoW, but explain level of indirection in issues
        // -- issue: sybil resistance
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