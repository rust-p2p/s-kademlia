//! NodeTable Logic
//! store for all the buckets
//! -- goal is to load-balance between buckets to minimize lookups in different scenarios
//! -- there are fundamental tradeoffs ie bias towards recency vs worste-case...
//!
//! TODO
//! - look into LRU vs TTL for this
//! - replacement cache referenced in original paper
//! -- TTL preferred probably, but would be nice to make it modular enough to support
//! multiple eviction policies...
//! - see pr1117 of rust-libp2p
//!
//! section 4.2 of the s/kad paper for cache eviction policy
use crate::bucket;
use crate::id; // specifically NodeId, NodeTableId?


/// # NodeTable
///
/// A simple node table that removes stale nodes
/// i.e. TTL based on `KEY_EXPIRATION` seconds
#[derive(Default)]
pub struct NodeTable<NodeTableId, NodeId> {
    table_id: NodeTableId,
    hash: DiscoHash,
    buckets: Vec<KBucket<NodeTableId, NodeId>>,
}


