//!
//! Configuration for participating in this protocol
use std::sync::{Arc, Mutex};

/// Local NodeConfig
#[derive(Clone)]
pub struct NodeConfig {
    data: NodeInfo,         // need to be sync
    routing_table: Table,   // need to be sync
    // block_store: BDDB, // use crate::bitswap::BDDB (it is like wrapper around in-memory cache for blocks)
    // -- other possible storage items --
    // storage: Arc<Mutex<Storage>>, // partition storage based on data type (red-blue)
    // pending_requests: Arc<Mutex<HashMap<Key, Sender<Response>>>>, // use libp2p Provider abstraction for pull response interface
    // is_active: Arc<AtomicBool>, // could make this a more complex config specifying
    // nuanced participation in various protocols `Arc<Protocol>`)
}

// look at service...

/// Find method
/// iterative find method (TODO: use bitswap)

/// use bitswap/strategy for this instead of whatever the fuck I've been doing

// Traits for NodeConfig
// impl WeakSignature for Node {}
// impl StrongSignature for Node {}