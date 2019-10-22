// Storage DHT
mod entry;
mod bucket;
mod table;

// cache design for many entries
use crate::key::KeyPair;
use std::hash::{Hash, Hasher};
// used wasm_timer::Instant;

// inspired by libp2p's record trait:
/// There are two types of records managed by a `RecordStore`:
///
///   1. Regular (value-)records. These records store an arbitrary value
///      associated with a key which is distributed to the closest nodes
///      to the key in the Kademlia DHT as per the standard Kademlia "push-model".
///      These records are subject to re-replication and re-publication as
///      per the standard Kademlia protocol.
///
///   2. Provider records. These records associate the ID of a peer with a key
///      who can supposedly provide the associated value. These records are
///      mere "pointers" to the data which may be followed by contacting these
///      providers to obtain the value. These records are specific to the
///      libp2p Kademlia specification and realise a "pull-model" for distributed
///      content. Just like a regular record, a provider record is distributed
///      to the closest nodes to the key.

// (1) can be used for outsiders joining to prove trusthworthiness at first
// (2) and participation in serving nodes achieves high publisher fairness
// ==> both influence the service routing bias in low availability scenarios 
// ----> biased towards somewhat trustworthy suppliers...