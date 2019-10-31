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
use crate::node::NodeInfo;
use crate::id::NodeId;
use disco::DiscoHash;
use std::collections::VecDeque;
use std::cmp; // for the find method, comparing distances

/// Number of Buckets in a Table
const BUCKET_COUNT: usize = 32;
/// Number of Nodes in a Bucket
const DEFAULT_NODE_SIZE: usize = 64;

pub struct Table<IpAddr> {
    buckets: Vec<Bucket<IpAddr>>,
    bucket_count: usize,
}

pub struct Bucket<IpAddr> {
    nodes: VecDeque<NodeInfo<IpAddr>>,
    node_count: usize,
}

impl<IpAddr> Table<IpAddr>
where
    // TODO: better type bound here based on chosen ip address format
    IpAddr: Clone + Copy,
{
    /// Create a new node table
    ///
    /// -> does table need ownership associated var like this_id?
    pub fn new(bucket_count: usize, node_count: usize) -> Table<IpAddr> {
        Table {
            buckets: (0..bucket_count).map(|_| Bucket::new(node_count)).collect(),
            bucket_count,
        }
    }

    pub fn buckets(&self) -> &Vec<Bucket<IpAddr>> {
        &self.buckets
    }
}

// add trait for BasicNodeTable
// - random_id (need to PR discohash)
// - update
// - find
// - pop-oldest

// TODO
//
// - faster to iterate over slice than vec, don't use vecs for all these storage items
// -- or take some of them by reference
impl<IpAddr> Bucket<IpAddr>
where
    // TODO: better type bound
    IpAddr: Clone + Copy,
{
    pub fn new(node_count: usize) -> Bucket<IpAddr> {
        // should enforce some lower bound for node count here
        Bucket {
            nodes: VecDeque::new(),
            node_count,
        }
    }

    pub fn nodes(&self) -> &VecDeque<NodeInfo<IpAddr>> {
        &self.nodes
    }

    pub fn size(&self) -> usize {
        self.size
    }

    /// Update position
    ///
    /// Adds new nodes and places old nodes at the top of the bucket if used
    pub fn update_node(&mut self, node: &NodeInfo<IpAddr>) -> bool {
        // is there a cleaner way of returning errors based on conditions, like the ensure macro in substrate?
        let full_bucket = self.nodes.len() == node.node_count;
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
            (_, _) => {
                self.promote_to_top(node.clone());
            }
        }
    }

    fn promote_to_top(&mut self, node: NodeInfo<IpAddr>) {
        let new_nodes = self
            .nodes
            .into_iter()
            // filter instead of retain
            .retain(|n| n != node.id)
            .push_back(node.clone());
        self.nodes = new_nodes;
    }

    pub fn find(&self, id: &NodeId, count: usize) -> Vec<NodeInfo<IpAddr>> {
        let mut nodes_copy: Vec<_> = self.nodes.into_iter().map(|n| n.clone()).collect();
        nodes_copy.sort_by_key(|n| Table::<IpAddr>::distance(id, &n.id));
        nodes_copy[0..cmp::min(count, nodes_copy.len())].to_vec()
    }
}
