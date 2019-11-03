use crate::node_id::NodeId;
use crate::store::{NodeTable, NodeBucket};
use std::{sync::{Arc, Mutex}, net::SocketAddr, cmp::PartialEq};

#[derive(Clone, Debug)]
pub struct NodeInfo {
    /// Identifier
    pub id: NodeId,
    /// IP Address, Port
    pub address: SocketAddr,
    /// Status of the node
    pub status: NodeStatus,
}

impl PartialEq<NodeId> for NodeInfo {
    fn eq(&self, other: &NodeId) -> bool {
        self.id == *other
    }
}

impl PartialEq for NodeInfo {
    fn eq(&self, other: &NodeInfo) -> bool {
        self.id == other.id
    }
}

/// Status
///
/// reveals connected or not connected status
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum NodeStatus {
    /// Node is connected.
    Connected,
    /// Node is not connected
    DisConnected,
}