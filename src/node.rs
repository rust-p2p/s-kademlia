//! Node
//!
// ! - research ways for embedding latency in this section
use crate::id::NodeId;

pub struct Node {
    pub local_node: NodeInfo,
    // could replicate data bases
    // phantomData
}

#[derive(Clone, Debug)]
pub struct NodeInfo<NodeId, NodeScore> {
    /// ID of the node.
    pub id: NodeId,
    /// Network address of the node. (is this synonymous with port)
    pub port: String,
    /// Score (reputation) of the node
    pub score: NodeScore,
}

/// *Reputation* (binary status atm)
///
/// - designed from the ground up with a field for sharing strategies for certain types of data
/// ...still being researched
/// - `Status` might contain more information
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum NodeScore<Status> {
    /// The node is considered connected.
    Connected(Status),
    /// Node is not connected
    NotConnected(Status),
} // T might represent some object with reputation-based information

// impl WeakSignature

// impl StrongSignature