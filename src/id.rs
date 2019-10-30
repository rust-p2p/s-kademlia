use crate::ed25519;
use disco::DiscoHash;
use time::{Duration, SteadyTime};
use crate::error::TimeOutError;
use crate::key::{Keypair, PublicKey};
// use std::{convert::TryFrom, fmt, str::FromStr};

// TODO: can this just be imported from disco?
pub fn hash(input: Vec<u8>) -> DiscoHash {
    let mut h = DiscoHash::new();
    h.write(input.as_bytes());
    h.sum()
}

/// NodeId
///
/// contains a DiscoHash
#[derive(Clone)]
pub struct NodeId {
    discohash: DiscoHash,
}

// TODO: does NodeId need to implement some other traits? Probably `Send` at the very least
unsafe impl Send for NodeId {}

impl NodeId {
    /// New NodeId
    ///
    /// No hardness mechanism for slowing id generation
    #[inline]
    fn new(pubkey: PublicKey) -> NodeId {
        let keyhash = hash(pubkey);
        return NodeId { keyhash }
    }

    /// Generate NodeId
    ///
    /// not sybil resistant by default
    #[inline]
    pub fn generate(pubkey: PublicKey) -> NodeId {
        NodeId::new(pubkey)
    }
    
    /// Generate NodeId with Resistance
    ///
    /// - requires hash of `PublicKey` to be have `difficulty` trailing zeros
    pub fn hard_generate(pubkey: PublicKey, difficulty: usize, timeout: usize) -> Result<NodeId, TimeOutError> {
        let clock = SteadyTime::now();
        loop {
            // TODO: replace with generation of PublicKey to hash and then choose an ID
            // - could be more generic, requiring some configuration
            let new_id = NodeId::new(pubkey);
            // default trailing zeros (remove `rev()` for leading zeros)
            let disco_iter = new_id.discohash.as_bytes().iter().rev();
            let success = true;
            for i in 0..difficulty {
                if disco_iter.nth(i) != 0 {
                    success = false;
                }
            }
            if success {
                return Ok(new_id)
            }
            //                           converts usize into i64 and panics if doesn't fit
            // TODO: consider if this is best or if I should just pass i64 as an argument
            if clock > Duration::seconds(timeout.try_into().unwrap()) {
                return Err(TimeOutError)
            }
        }
    }

    #[inline]
    fn digest(&self) -> &[u8] {
        self.discohash.as_bytes()
    }

    #[inline]
    fn is_public_key(&self, pubkey: &PublicKey) -> bool {
        self.discohash == hash(pubkey)
    }

    #[inline]
    pub fn random(output_len: usize) -> NodeId {
        NodeId {
            discohash: DiscoHash::random(output_len),
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

#[cfg(test)]
mod tests {
    todo!();
}
