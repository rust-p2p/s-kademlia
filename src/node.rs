use crate::id::NodeId;

#[derive(Clone, Debug)]
pub struct Node<IPAddr, Port, NodeId> {
    /// Network address of the node.
    pub address: IpAddr,
    /// Port of the node
    pub port: Port,
    /// ID of the node.
    pub id: NodeId,
}

// use state to generate weak signature in `id`
// - think about where to place this logic (here or in id)
// intended to be done on `Node`s
pub trait WeakSignature {
    fn weaksignature(ip: IPAddr, port: Port, node_id: NodeId) -> Node<IPAddr, Port, NodeId>;
}