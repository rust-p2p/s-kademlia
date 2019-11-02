//!
//! Configuration for participating in this protocol
use std::sync::{Arc, Mutex};

/// Local NodeConfig
#[derive(Clone)]
pub struct NodeConfig {
    data: Arc<NodeInfo>,
    routing_table: Arc<Mutex<Table>>,
    // -- other possible storage items --
    // storage: Arc<Mutex<Storage>>, // partition storage based on data type (red-blue)
    // pending_requests: Arc<Mutex<HashMap<Key, Sender<Response>>>>, // use libp2p Provider abstraction for pull response interface
    // is_active: Arc<AtomicBool>, // could make this a more complex config specifying
    // nuanced participation in various protocols `Arc<Protocol>`)
}

// Traits for NodeConfig
// impl WeakSignature for Node {}
// impl StrongSignature for Node {}