use crate::ed25519;
use disco::symmetric::DiscoHash;
// use uint::U256;
use crate::key::{Keypair, PublicKey}; // KeyPair, PublicKey
// use std::{convert::TryFrom, fmt, str::FromStr};

/// AntiSybilMechanism (wip)
///
/// - each variant will require public functions describing
/// the mechanism in question (with benchmarks in benches/pow)
pub enum AntiSybilMechanism {
    TrailingZerosHash(usize), // TODO
    LeadingZerosHash(usize), // ""
}

// TODO: can this just be imported from disco?
pub fn hash(input: Vec<u8>) -> DiscoHash {
    let mut h = DiscoHash::new();
    h.write(input.as_bytes());
    h.sum()
}

/// NodeId
///
/// contains a DiscoHash
#[derive(Clone, Copy, Debug)]
pub struct NodeId {
    discohash: disco::DiscoHash,
}

impl NodeId {
    /// Generate NodeId
    ///
    /// not sybil resistant by default
    #[inline]
    pub fn generate(pubkey: PublicKey) -> NodeId {
        // call more specific generation with None resistance
        generate_with_resistance(pubkey, None)
    }
    
    /// Validate NodeId
    ///
    /// - demonstrates that the node_id is valid
    /// - TODO: verify generate_with_resistance() generation ;)
    #[inline]
    fn validate(&self) -> bool {
        self.keyhash == hash(self.pubkey)
    }

    #[inline]
    pub fn random(output_len: usize) -> NodeId {
        NodeId {
            discohash: disco::DiscoHash::random(output_len)
        }
    }
}

impl From<PublicKey> for NodeId {
    #[inline]
    fn from(key: PublicKey) -> NodeId {
        NodeId::generate(key)
    }
}

// TODO:
// - TryFrom for NodeId
// - TryFrom<Vec<u8>> for NodeId
// - PartialEq<NodeId> for NodeId
// - PartialEq<NodeId> for DiscoHash
// - AsRef<DiscoHash> for NodeId
// - AsRef<[u8]> for NodeId
// - Into<DiscoHash> for NodeId
// - FromStr for NodeId

/// Generate NodeId with Resistance
///
/// as per s-kademlia, applies some resistance to NodeId generation like a decentralized cryptopuzzle
fn generate_with_resistance(pubkey: PublicKey, anti_sybil: Option<AntiSybilMechanism>) -> NodeId {
    if let Some(puzzle) = anti_sybil {
        // match on puzzle
        // define each outcome as a function
        todo!()
    } else {
        let keyhash = hash(pubkey);
        return NodeId { keyhash }
    }
}

#[cfg(test)]
mod tests {
    todo!();
}
