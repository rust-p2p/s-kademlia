//! AntiSybilMechanism
//!
//! s-kademlia encourages applying a crypto puzzle to NodeId generation
//! to mitigate DoS-based overflow attacks by new joiners
use bencher::black_box;
use bencher::Bencher;

fn leadingzeros_discohash(difficulty: usize, B: &Bencher) {
    // difficulty will effect timeout
    loop {
        // generate NodeId
    }
}

fn trailingzeros_discohash(difficulty: usize, B: &Bencher) {
    // difficulty will effect timeout
    loop {
        // generate NodeId
    }
}

// group them in the same benchmark
// - notice: this studies the distribution of the output for discohash