use crate::node_id::NodeId;
use crate::store::{Table, Bucket};
use std::{sync::{Arc, Mutex}, net::IpAddr};

#[derive(Clone, Debug)]
pub struct NodeInfo {
    /// Identifier
    pub id: NodeId,
    /// Address
    pub address: IpAddr,
    /// Status of the node
    pub status: NodeStatus,
}

impl PartialEq<NodeId> for NodeInfo {
    fn eq(&self, other: NodeId) -> bool {
        let other_iter = other.discohash.iter();
        let count: usize = 0;
        self.discohash.into_iter().for_each(|i| {
            // bitwise comparison
            if other_iter.nth(count) != i {
                return false
            }
            count += 1;
        });
        true
    }
}

/// Kadelia uses bitwise xor for its metric space
pub trait XORMetric {
    type Metric: SimpleArithmetic;

    fn distance(&self, other: &Self) -> Self::Metric;
}

impl XORMetric<Vec<u8>> for NodeInfo {
    fn distance(&self, other: &Self) -> Self::Metric  {
        // TODO: if lengths are different, return an error
        let this_node_iter = self.node_id.discohash.iter();
        let count: usize = 0;
        let other_iter: Vec<u8> = other.discohash.into_iter().map(|i| {
            let result_bit = this_node_iter.nth(count) ^ i;
            count += 1;
            result_bit
        }).collect()
    }
}

/// Status
///
/// reveals connected or not connected status
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum NodeStatus {
    /// Node is connected.
    Connected,
    /// Node is not connected
    DisConnected,
}