//! (unfinished) example of querying in this kademlia protocol
use rand;
use ed25519_dalek::Keypair;

fn main() {
    // use to generate NodeId and sign associated messages
    let local_key = Keypair::generate(&mut rand::thread_rng());
}