//! Storage (Routing Table)
use crate::error::DistanceIsZero;
use crate::node::{NodeInfo, NodeStatus};
use crate::node_id::{KadMetric, NodeId};
use std::{cmp, collections::VecDeque};

/// Number of Buckets in a NodeTable
const DEFAULT_BUCKET_COUNT: usize = 32;
/// Number of Nodes in a NodeBucket
const DEFAULT_BUCKET_SIZE: usize = 64;

#[derive(Clone, Debug)]
pub struct NodeTable {
    /// The `NodeId` identifying the local peer that owns the routing table
    pub id: NodeId,
    /// The buckets comprising the routing table
    pub buckets: Vec<NodeBucket>,
}

impl NodeTable {
    /// Create a new node table
    pub fn new(id: NodeId) -> Self {
        NodeTable::new_custom_table(id, DEFAULT_BUCKET_COUNT, DEFAULT_BUCKET_SIZE)
    }

    pub fn new_custom_table(id: NodeId, bucket_count: usize, node_count: usize) -> Self {
        NodeTable {
            id,
            buckets: (0..bucket_count)
                .map(|_| NodeBucket::new(node_count))
                .collect(),
        }
    }

    /// Get buckets
    pub fn buckets(&self) -> &Vec<NodeBucket> {
        &self.buckets
    }

    fn bucket_index(&self, id: &NodeId) -> Result<usize, DistanceIsZero> {
        let diff = self.id.distance(&id)?;
        // this error returned from `ok_or` is actually inaccurate
        let index = (self.buckets.len() - diff.leading_zeros() as usize)
            .checked_sub(1)
            .ok_or(DistanceIsZero);
        index
    }

    pub fn update(&mut self, node: &NodeInfo) -> bool {
        assert!(node.id != self.id);
        let bucket = self.bucket_index(&node.id).unwrap();
        self.buckets[bucket].update(node)
    }

    pub fn find(&self, id: &NodeId, count: usize) -> Vec<NodeInfo> {
        assert!(count > 0 && *id != self.id);

        let mut nodes_found: Vec<_> = self
            .buckets
            .iter()
            .flat_map(|b| &b.nodes)
            .map(|n| n.clone())
            .collect();
        nodes_found.sort_by_key(|n| n.id.distance(&id).unwrap());
        nodes_found[0..cmp::min(count, nodes_found.len())].to_vec()
    }

    pub fn pop_oldest(&mut self) -> Vec<NodeInfo> {
        self.buckets
            .iter_mut()
            .filter(|b| !b.nodes.is_empty() && b.node_count == b.nodes.len())
            .map(|b| b.nodes.pop_front().unwrap())
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct NodeBucket {
    pub nodes: VecDeque<NodeInfo>,
    pub node_count: usize,
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
                // replace bool return value with Result and specific error type?
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
        let mut all_nodes = self
            .nodes
            .iter()
            // filter out node in question
            .filter(|n| n.id != node.id)
            .cloned()
            .collect::<VecDeque<NodeInfo>>();
        // push to the tail of the list
        all_nodes.push_back(node.clone());
        self.nodes = all_nodes;
    }

    // TODO: only include count if searching for multiple ids
    pub fn find(&self, id: &NodeId, count: usize) -> Vec<NodeInfo> {
        let mut nodes_copy: Vec<_> = self.nodes.iter().map(|n| n.clone()).collect();
        // TODO: propagate `found_self` error instead of unwrapping
        nodes_copy.sort_by_key(|n| n.id.distance(&id).unwrap());
        nodes_copy[0..cmp::min(count, nodes_copy.len())].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::{NodeBucket, NodeTable};
    use crate::node::NodeInfo;
    use crate::node_id::{KadMetric, NodeId};

    // TODO
}
