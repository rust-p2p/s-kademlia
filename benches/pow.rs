//! anti-sybil mechanism
//! for publc keypair generations
//! vs id genersation
//! -- and the chosen algorithm for the latter

use bencher::Bencher;
use bencher::black_box;

// Ideas (rando)
// - add bloom filter for finding addresses
// - add cuckoo hash pow
// - replace bloom filter for finding addresses, layer with cuckoo hash

fn using_std_time(B: &Bencher) {
    let super_counter = SteadyTime::now()
    loop {
        let new_keypair = ed25519::Keypair::generate(&mut rand::thread_rng());
        let difficulty: u32 = 5; // number of required trailing zeros
        let counter = 0u32;
        let success = true;
        new_keypair.to_bytes().into_iter().rev().for_each(|i| {
            if counter < difficulty && i != 0 {
                success = false;
            }
            counter += 1;
        });
        if success { return Ok(Keypair(new_keypair)) };
        super_counter += 1;
        if super_counter > timeout {
            return Err(AntiSybilError::new("Anti-sybil mechanism timed out"))
        }
    };
}

