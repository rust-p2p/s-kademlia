//! Sybil Mechanism Testing
#[macro_use]
extern crate bencher;
use crate::node_id::NodeId;
use bencher::black_box;
use bencher::Bencher;

/// Generate NodeId with Resistance
///
/// Requires disco::hash(public_key) to be have `difficulty` number of trailing zeros
/// WARNING: loop could keep running for a long time (no benchmarking done yet)
pub fn hard_generate(difficulty: usize) -> NodeId {
    loop {
        let new_id = NodeId::generate();
        let mut success = true;
        // default leading zeros
        for i in 0..difficulty {
            if new_id.discohash.get(i).unwrap() != &0u8 {
                success = false;
            }
        }
        if success {
            return new_id;
        }
    }
}

fn leadingzeros_discohash(difficulty: usize, B: &Bencher) {
    // difficulty will effect timeout
    loop {
        // generate NodeId w/ hard_generate logic
    }
}

fn main() {
    // <study distribution of output for discohash(pubkey)>
}
