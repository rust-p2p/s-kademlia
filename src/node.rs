use crate::node_id::{NodeId, KadMetric};
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

// TODO: is there a way to just check if every field is equal?
impl PartialEq for NodeInfo {
    fn eq(&self, other: &NodeInfo) -> bool {
        self.id == other.id && self.address == other.address && self.status == other.status
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
    Disconnected,
}

#[cfg(test)]
mod tests {
    use super::{NodeInfo, NodeStatus};

    // test that node status 
}