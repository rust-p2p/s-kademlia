use crate::id::NodeId;

#[derive(Clone, Debug)]
pub struct Node<IPAddr, NodeId> {
    /// Network address of the node.
    pub address: IpAddr, // often used synonymously with port
    /// Port of the node
    pub port: String,
    /// ID of the node.
    pub id: NodeId,
    /// Score (reputation) of the node
    pub score: NodeScore

}
// todo
// - do address and port have to be differen things
// - what other data is valuable at this layer for embedding latency??? add marker trait that follows `NodeId` or the other way around...


/// see incentives work and could manage here? (open issue)
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum NodeScore<T: /*some low-friction arithmetic type for scoring (holds strategy for nodes)*/> {
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
// standard impl based on signing with public key associated with `NodeId`

// strong signature also here
// -- will require signing with *a* public key associated with the `NodeId` `=>` how is this verified efficiently?
pub trait StrongSignature<M> {
    fn strong_signature(message: &M);
}