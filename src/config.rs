//!
//! Configuration for participating in this protocol
use std::sync::{Arc, Mutex};
use crate::node::NodeInfo;
use crate::store::{NodeTable, NodeBucket};

// use `efcp/efcp`

/// Local NodeConfig
#[derive(Clone)]
pub struct NodeHandler {
    data: NodeInfo,         // need to be sync
    routing_table: NodeTable,   // need to be sync
    // block_store: BDDB, // use crate::bitswap::BDDB (it is like wrapper around in-memory cache for blocks)
    // -- other possible storage items --
    // storage: Arc<Mutex<Storage>>, // partition storage based on data type (red-blue)
    // pending_requests: Arc<Mutex<HashMap<Key, Sender<Response>>>>, // use libp2p Provider abstraction for pull response interface
    // is_active: Arc<AtomicBool>, // could make this a more complex config specifying
    // nuanced participation in various protocols `Arc<Protocol>`)
}

// look at service...

// Find method
// iterative find method (TODO: use bitswap)

// Traits for NodeConfig
// impl WeakSignature for Node {}
// impl StrongSignature for Node {}