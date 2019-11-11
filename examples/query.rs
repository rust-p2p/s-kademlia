//! (unfinished) example of querying in this kademlia protocol
use rand;
use ed25519_dalek::Keypair;
use s_kademlia::NodeId;

fn main() {
    // use to generate NodeId and sign associated messages
    let local_key = Keypair::generate(&mut rand::thread_rng());
    let node_id = NodeId::from_public_key(local_key.public);
}