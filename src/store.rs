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
use crate::node_id::{NodeId, KadMetric};
use std::{collections::VecDeque, cmp};
// use disco::hash;

/// Number of Buckets in a NodeTable
const DEFAULT_BUCKET_COUNT: usize = 32;
/// Number of Nodes in a NodeBucket
const DEFAULT_BUCKET_SIZE: usize = 64;

pub struct NodeTable {
    id: NodeId,
    buckets: Vec<NodeBucket>,
    bucket_count: usize,
}

impl NodeTable {
    /// Create a new node table
    pub fn new(id: NodeId) -> Self {
        NodeTable::new_dynamic_table(id, DEFAULT_BUCKET_COUNT, DEFAULT_BUCKET_SIZE)
    }

    pub fn new_dynamic_table(id: NodeId, bucket_count: usize, node_count: usize) -> Self {
        NodeTable {
            id,
            buckets: (0..bucket_count).map(|_| NodeBucket::new(node_count)).collect(),
            bucket_count,
        }
    }

    /// Get buckets
    pub fn buckets(&self) -> &Vec<NodeBucket> {
        &self.buckets
    }

    pub fn bucket_count(&self) -> usize {
        self.bucket_count
    }

    fn bucket_number(&self, id: &NodeId) -> usize {
        let diff = self.id.discohash.distance(&id.discohash);

        assert!(diff.is_zero());
        // TODO: this is kind of a placeholder, should be more clear
        diff.bits() - 1
    }

    pub fn update(&mut self, node: &NodeInfo) -> bool {
        assert!(node.id != self.id);
        let bucket = self.bucket_number(&node.id);
        self.buckets[bucket].update(node)
    }

    pub fn find(&self, id: &NodeId, count: usize) -> Vec<NodeInfo> {
        assert!(count > 0 && *id != self.id);

        let mut nodes_found: Vec<_> = self.buckets.iter().flat_map(|b| &b.nodes)
                                                    .map(|n| n.clone())
                                                    .collect();
        nodes_found.sort_by_key(|n| n.id.discohash.distance(&id.discohash));
        nodes_found[0..cmp::min(count, nodes_found.len())].to_vec()
    }

    pub fn pop_oldest(&mut self) -> Vec<NodeInfo> {
        // TODO: TTL (and more generic) cache
        self.buckets
            .iter_mut()
            .filter(|b| !b.nodes.is_empty() && b.node_count == b.nodes.len())
            .map(|b| b.nodes.pop_front().unwrap())
            .collect()
    }
}

pub struct NodeBucket {
    // TODO: make into VecDequeue if aligns with eviction policy
    nodes: VecDeque<NodeInfo>,
    node_count: usize,
}

impl NodeBucket {
    pub fn new(node_count: usize) -> NodeBucket {
        NodeBucket {
            nodes: VecDeque::new(),
            node_count,
        }
    }

    pub fn nodes(&self) -> &VecDeque<NodeInfo> {
        &self.nodes
    }

    pub fn node_count(&self) -> usize {
        self.node_count
    }

    /// Update position
    ///
    /// Adds new nodes and places old nodes at the top of the bucket if used
    pub fn update(&mut self, node: &NodeInfo) -> bool {
        let full_bucket = self.nodes.len() == self.node_count;
        let in_bucket = self.nodes.contains(&node);
        match (full_bucket, in_bucket) {
            (true, false) => {
                // TODO: add new kbucket and do any necessary reordering
                // replace bool return value with Result and specific error type
                false
            }
            (false, true) => {
                // add node to bucket with room
                self.nodes.push_back(node.clone());
                true
            }
            _ => {
                self.promote_to_top(node.clone());
                true
            }
        }
    }

    fn promote_to_top(&mut self, node: NodeInfo) {
        let mut all_nodes = self.nodes
                            .iter()
                            // filter out node in question
                            .filter(|n| n.id != node.id)
                            .cloned()
                            .collect::<VecDeque<NodeInfo>>();
        // push to the tail of the list
        all_nodes.push_back(node.clone());
        self.nodes = all_nodes;
    }

    pub fn find(&self, id: &NodeId, count: usize) -> Vec<NodeInfo> {
        let mut nodes_copy: Vec<_> = self.nodes.iter().map(|n| n.clone()).collect();
        nodes_copy.sort_by_key(|n| n.id.discohash.distance(&id.discohash));
        nodes_copy[0..cmp::min(count, nodes_copy.len())].to_vec()
    }
}

// Eviction Policy
//
// TODO: least-recently seen eviction policy, except live nodes are never removed from the list
// When a kademlia node receives any message (request or reply)
// from another node, it updates the appropriate k-bucket for the sender's
// nodeID. 
// - If the sending node already exists in the recipient's k-bucket and the bucket
// has fewer than k entries, then the recipient just inserts the new sender at the tail
// of the list.
// - If the appropriate k-bucket is full, then the recipient pings the k-bucket's
// least recently seen node. 
// -- If it fails to respond, it's evicted and new node is inserted
// -- else (if it responds), the least recently seen node is moved to the tail
// of the list, and the new sender's contact is discarded