/// replacement cache based on original paper
/// see pr1117 on rl2p
use super::*;

use crate::bucket;

// see cacache impl for async read/write
// also see rust ttl structure
// amd pr1117 of rust-libp2p