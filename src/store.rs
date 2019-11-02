//! Storage (Routing Table)
//! to be abstracted into more modular storage traits
//! - see dynamic-sized arrays vs static routing tables
//! (overarching goal is to partition DHT based on data type)
//! - should use associated `PROVIDER` abstraction from libp2p
//!
//! Long-Term TODO: abstract storage containers into traits and macros like in Substrate
//! -- vision is a network topology that adapts according to voting/gossip by nodes
//! -- different data store for bloom filter cache for r5n
//! -- different data store for PeerId membership via Brahms gossip
use crate::node::{NodeInfo, NodeStatus};
use crate::node_id::NodeId;
use disco::hash;
use std::collections::VecDeque;
use std::cmp; // for the find method, comparing distances

/// Number of Buckets in a Table
const BUCKET_COUNT: usize = 32;
/// Number of Nodes in a Bucket
const DEFAULT_NODE_SIZE: usize = 64;
// TODO: define more methods for Table and Bucket for specific sized stores

pub struct Table {
    buckets: Vec<Bucket>,
    bucket_count: usize,
}

pub struct Bucket {
    nodes: VecDeque<NodeInfo>,
    node_count: usize,
}

impl Table {
    /// Create a new node table
    ///
    /// -> does table need ownership associated var like this_id?
    pub fn new(bucket_count: usize, node_count: usize) -> Table {
        Table {
            buckets: (0..bucket_count).map(|_| Bucket::new(node_count)).collect(),
            bucket_count,
        }
    }

    /// Get buckets
    pub fn buckets(&self) -> VecDeque<Bucket> {
        &self.buckets
    }

    pub fn bucket_count(&self) -> usize {
        &self.bucket_count
    }

    // pub fn update(&self) {

    // }

    // pub fn find(&self) {

    // }

    // pub fn pop_oldest(&self) {

    // }
}

impl Bucket {
    pub fn new(node_count: usize) -> Bucket {
        Bucket {
            nodes: VecDeque::new(),
            node_count,
        }
    }

    pub fn nodes(&self) -> &VecDeque<NodeInfo> {
        &self.nodes
    }

    pub fn node_count(&self) -> usize {
        &self.node_count
    }

    /// Updates the status of the node referred to by the given key,
    /// if in the bucket
    pub fn update_status(&mut self, status: NodeStatus) {
        // Remove the node from current position and reinsert it
        // with the desired status, which puts it at the end of either the 
        // prefix list of disconnected nodes or the suffix list of connected
        // nodes (most recently disconnected...)
        if let Some(pos) = self.position(key)
    }

    /// Update position
    ///
    /// Adds new nodes and places old nodes at the top of the bucket if used
    pub fn update_node(&mut self, node: &NodeInfo) -> bool {
        // is there a cleaner way of returning errors based on conditions, like the ensure macro in substrate?
        let full_bucket = self.nodes.len() == node.node_count();
        let in_bucket = self.nodes.contains(&node);
        match (full_bucket, in_bucket) {
            (true, false) => {
                // add new kbucket and do any necessary reordering
                todo!();
            }
            (false, true) => {
                // add node to bucket with room
                self.nodes.push_back(node.clone());
                true
            }
            _ => {
                self.promote_to_top(node.clone());
            }
        }
    }

    fn promote_to_top(&mut self, node: NodeInfo) {
        let new_nodes = self.nodes
                            .into_iter()
                            // take out nodes that aren't this node
                            .filter(|n| n.id != node.id)
                            .collect()
                            .push_back(node.clone());
        self.nodes = new_nodes;
    }

    pub fn find(&self, id: &NodeId, count: usize) -> Vec<NodeInfo> {
        let mut nodes_copy: Vec<_> = self.nodes.into_iter().map(|n| n.clone()).collect();
        nodes_copy.sort_by_key(|n| Table::distance(id, &n.id));
        nodes_copy[0..cmp::min(count, nodes_copy.len())].to_vec()
    }
}
