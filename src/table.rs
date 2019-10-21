// [Implementation Notes]
//
// 1. Routing Table Layout
//
// The routing table is currently implemented as a fixed-size "array" of
// buckets, ordered by increasing distance relative to a local key
// that identifies the local peer. This is an often-used, simplified
// implementation that approximates the properties of the b-tree (or prefix tree)
// implementation described in the full paper [0], whereby buckets are split on-demand.
// This should be treated as an implementation detail, however, so that the
// implementation may change in the future without breaking the API.





//! Routing Table Layout
//! - should be treated as an implementation detail s.t. future changes do not break the core API (modular piece)
//! - the properties of the b-tree or prefix tree are relevant to how the distance must be stored
//! --> ensure buckets can be split on-demand?
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


