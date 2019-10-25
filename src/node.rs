//! Node
//!
//! - research ways for embedding latency in this section
//!
//! ...note that protobufs are used in lieu of peer_ids

use crate::id::NodeId;
use std::sync::{Arc, Mutex};
use crate::store::Table;

/// *Reputation* (binary status, connected or not)
///
///
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum NodeScore<Status> {
    /// The node is considered connected.
    Connected(Status),
    /// Node is not connected
    NotConnected(Status),
} // T might represent some object with reputation-based information

#[derive(Clone, Debug)]
pub struct NodeInfo<Hash> {
    /// ID of the node.
    pub id: NodeId<Hash>,
    /// Network address of the node
    pub port: String, // same as address? or when is each used?
                      // Score (reputation) of the node
                      // pub score: NodeScore,
}

/// Local NodeConfig
#[derive(Clone)]
pub struct NodeConfig {
    node_data: Arc<NodeInfo<DiscoHash>>,
    routing_table: Arc<Mutex<Table<NodeInfo<DiscoHash>>>>,
    // -- other possible storage items --
    // storage: Arc<Mutex<Storage>>, // partition storage based on data type (red-blue)
    // pending_requests: Arc<Mutex<HashMap<Key, Sender<Response>>>>, // use libp2p Provider abstraction for pull response interface
    // is_active: Arc<AtomicBool>, // could make this a more complex config specifying
    // nuanced participation in various protocols `Arc<Protocol>`)
}

// Traits for Node
// impl WeakSignature for Node {}

// impl StrongSignature for Node {}
