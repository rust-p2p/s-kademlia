//! Handler for Participating Nodes
use crate::node::NodeInfo;
use crate::store::{NodeBucket, NodeTable};
use std::sync::{Arc, RwLock};

/// Local NodeConfig
#[derive(Clone)]
pub struct NodeHandler {
    id: NodeInfo,
    table: Arc<RwLock<NodeTable>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn sign_and_verify() {
        // sign a message
        // verify that the signature works
    }
}