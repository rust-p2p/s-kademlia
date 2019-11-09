// utilities
// (1) any traits (added after minimal impl finished)

// (2) test scaffolding
#[cfg(test)]
pub mod test {
    use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    use std::collections::VecDeque;
    use crate::node_id::NodeId;
    use crate::store::{NodeTable, NodeBucket};
    use crate::node::{NodeInfo, NodeStatus};

    pub static ADDR: &'static str = "127.0.0.1:8008";

    // TODO: eventually add `net::SocketAddr` and `NodeStatus` as input parameters
    pub fn new_node_info(id: NodeId) -> NodeInfo {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let status = NodeStatus::Connected;
        let new_node = NodeInfo { id, socket, status };
        new_node
    }

    #[test]
    fn new_node_info_succeeds() {
        let id = NodeId::generate().unwrap();
        let new_node = new_node_info(id);
        assert_eq!(new_node.port(), 8080);
        assert_eq!(new_node.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    }

    fn new_node_bucket(node_count: usize) -> NodeBucket {
        let mut bucket = NodeBucket { nodes: VecDeque::new(), node_count };
        // should prevent duplicate NodeId generation eventually
        for i in 0..node_count {
            let id = NodeId::generate().unwrap();
            let new_node = new_node_info(id);
            bucket.nodes.push_back(new_node);
        }
        bucket
    }

    #[test]
    fn new_node_bucket_succeeds() {
        let new_bucket = new_node_bucket(10);
        assert_eq!(new_bucket.nodes.len(), 10);
    }
}