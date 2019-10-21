use crate::id::NodeId;

// port is probably a string...
#[derive(Clone, Debug)]
pub struct Node<IPAddr, Port, NodeId> {
    /// Network address of the node.
    pub address: IpAddr,
    /// Port of the node
    pub port: Port,
    /// ID of the node.
    pub id: NodeId,
    // each node is going to store at least one table as well...
}

/// see incentives work and could manage here? (open issue)
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum NodeScore<T> {
    /// The node is considered connected.
    Connected(T),
    /// Node is not connected
    NotConnected(T),
} // T might represent some object with reputation-based information

// use state to generate weak signature in `id`
// - think about where to place this logic (here or in id)
// intended to be done on `Node`s
pub trait WeakSignature {
    fn weak_signature(ip: IPAddr, port: Port, node_id: NodeId) -> Node<IPAddr, Port, NodeId>;
}
// strong signature also here
// -- will require signing with *a* public key associated with the `NodeId` `=>` how is this verified efficiently?
pub trait StrongSignature<M> {
    fn strong_signature(message: &M);
}