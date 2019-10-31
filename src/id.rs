use bs58;
use disco::{hash, DiscoHash};
use time::{Duration, SteadyTime};
use std::{convert::TryFrom, fmt, str::FromStr};
use crate::ed25519::{PublicKey};
use crate::error::TimeOutError;

/// NodeId
///
/// contains a DiscoHash
#[derive(Clone)]
pub struct NodeId {
    discohash: Vec<u8>,
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

unsafe impl Send for NodeId {}

impl NodeId {
    /// Generate NodeId
    ///
    /// Default no hard mechanism for slowing id generation
    #[inline]
    pub fn generate() -> NodeId {
        // TODO: private key must be stored somewhere for signing messages?
        let key = ed25519::Keypair::generate(&mut rand::thread_rng());
        let keyhash = hash(key.public.to_bytes(), 32);
        return NodeId { discohash: keyhash }
    }
    
    /// Generate NodeId with Resistance
    ///
    /// - requires hash of `PublicKey` to be have `difficulty` trailing zeros
    pub fn hard_generate(pubkey: PublicKey, key_len: usize, difficulty: usize, timeout: usize) -> Result<NodeId, TimeOutError> {
        let clock = SteadyTime::now();
        loop {
            // TODO: replace with generation of PublicKey to hash and then choose an ID
            // - could be more generic, requiring some configuration
            let new_id = NodeId::generate();
            // default trailing zeros (remove `rev()` for leading zeros)
            let disco_iter = new_id.discohash.iter().rev();
            let success = true;
            for i in 0..difficulty {
                if disco_iter.nth(i).unwrap() != &0u8 {
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
        &self.discohash.as_slice()
    }

    #[inline]
    fn is_public_key(&self, pubkey: PublicKey) -> bool {
        let ret_val = true;
        let mut counter = 0;
        let pk_hash = hash(pubkey.as_bytes(), 32).iter();
        &self.discohash.into_iter().for_each(|i| {
            // TODO: unsafe, check equal length first
            if pk_hash.nth(counter).unwrap() != &i {
                ret_val = false
            }
            counter += 1;
        });
        ret_val
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
    fn from(pubkey: PublicKey) -> NodeId {
        let keyhash = hash(pubkey.as_bytes(), 32);
        NodeId { discohash: keyhash }
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
    use crate::{NodeId, id};

    #[test]
    fn node_id_is_public_key() {
        let key = id::Keypair::generate().public();
        let node_id = key.clone().into_node_id();
        assert_eq!(peer_id.is_public_key(&key), Some(true));
    }

    #[test]
    fn peer_id_into_bytes_then_from_bytes() {
        let peer_id = identity::Keypair::generate_ed25519().public().into_peer_id();
        let second = PeerId::from_bytes(peer_id.clone().into_bytes()).unwrap();
        assert_eq!(peer_id, second);
    }

    #[test]
    fn peer_id_to_base58_then_back() {
        let peer_id = identity::Keypair::generate_ed25519().public().into_peer_id();
        let second: PeerId = peer_id.to_base58().parse().unwrap();
        assert_eq!(peer_id, second);
    }

    #[test]
    fn random_peer_id_is_valid() {
        for _ in 0 .. 5000 {
            let peer_id = PeerId::random();
            assert_eq!(peer_id, PeerId::from_bytes(peer_id.clone().into_bytes()).unwrap());
        }
    }
}
