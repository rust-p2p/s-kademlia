// utilities
// (1) any traits (added after minimal thing finished)


// (2) test scaffolding
#[cfg(test)]
pub mod test {
    use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    use std::collections::VecDeque;
    use super::super::node_id::NodeId;
    use super::super::store::{NodeTable, NodeBucket};
    use super::super::node::{NodeInfo, NodeStatus};

    pub static ADDR: &'static str = "127.0.0.1:8008";

    fn new_node_bucket() -> NodeBucket {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let id = NodeId::generate().unwrap();
        let status = NodeStatus::Connected;
        let new_node = NodeInfo { id, socket, status };
        let mut bucket = NodeBucket { nodes: VecDeque::new(), node_count: 0 };
        bucket
    }

    #[test]
    fn best() {
        assert!(true)
    }
}