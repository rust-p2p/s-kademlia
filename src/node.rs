use crate::node_id::{KadMetric, NodeId};
use crate::store::{NodeBucket, NodeTable};
use std::{
    cmp::PartialEq,
    net::{SocketAddr, IpAddr},
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub struct NodeInfo {
    /// Identifier
    pub id: NodeId,
    /// IP Address, Port
    pub socket: SocketAddr,
    /// Status of the node
    pub status: NodeStatus,
}

// TODO: is there a way to just check if every field is equal?
impl PartialEq for NodeInfo {
    fn eq(&self, other: &NodeInfo) -> bool {
        self.id == other.id && self.socket == other.socket && self.status == other.status
    }
}

impl NodeInfo {
    /// Get node's IpAddr
    pub fn ip(&self) -> IpAddr {
        self.socket.ip()
    }

    /// Get node's port
    pub fn port(&self) -> u16 {
        self.socket.port()
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
    use crate::node_id::NodeId;
    use std::net::{SocketAddr, IpAddr, Ipv4Addr};

    #[test]
    fn construct_node_info_succeeds() {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let id = NodeId::generate().unwrap();
        let status = NodeStatus::Connected;
        let new_node = NodeInfo { id, socket, status };
        assert_eq!(new_node.port(), 8080);
        assert_eq!(new_node.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    }
}
