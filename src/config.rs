//! Configuration for participating in this protocol
use crate::node::NodeInfo;
use crate::store::{NodeBucket, NodeTable};
use std::sync::{Arc, RwLock};

/// Local NodeConfig
#[derive(Clone)]
pub struct NodeHandler {
    id: NodeInfo,
    table: Arc<RwLock<NodeTable>>,
}

// look at `service.rs`
