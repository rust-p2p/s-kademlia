use crate::id::NodeId;
use disco::DiscoHash;
use std::sync::{Arc, Mutex};
use crate::store::Table;
use std::net::IpAddr;
// - \E async_std::net::IpAddr?

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
pub struct NodeInfo<IpAddr> {
    /// Identifier
    pub id: NodeId,
    /// Address of the node
    pub addr: IpAddr,
}

/// Local NodeConfig
#[derive(Clone)]
pub struct NodeConfig {
    data: Arc<NodeInfo<IpAddr>>,
    routing_table: Arc<Mutex<Table<NodeInfo<IpAddr>>>>,
    // -- other possible storage items --
    // storage: Arc<Mutex<Storage>>, // partition storage based on data type (red-blue)
    // pending_requests: Arc<Mutex<HashMap<Key, Sender<Response>>>>, // use libp2p Provider abstraction for pull response interface
    // is_active: Arc<AtomicBool>, // could make this a more complex config specifying
    // nuanced participation in various protocols `Arc<Protocol>`)
}

// Traits for Node
// impl WeakSignature for Node {}

// impl StrongSignature for Node {}

// ------ METRIC SPACE (distance) ------
pub trait MetricSpace: Sized {
    type Metric: Copy + Clone + PartialOrd;

    fn distance(self, other: Self) -> Self::Metric;
}