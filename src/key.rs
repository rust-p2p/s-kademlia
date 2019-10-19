use crate::ed25519;
pub use crate::ed25519::Signature;
use crate::x25519;

// TODO:
// - standard conversions for Encode and Decode
// choose how to serialize independent of above..
// - impl `weak` and `strong` signatures with different inputs

#[derive(Clone)]
pub struct Key<'a> {
    public_key: ed25519::PublicKey,
    secret_key: &'a ed25519::Keypair,
    // associated id? for (1) and (2)
}

impl<'a> Key<'a> {
    // generate a keypair
    pub fn new<T: Into<SecretKey<'a>>>(secret: T) -> Self {
        // not understanding how David's code works...
        // -- just need to generate priv, public pair for these functionalities

        let secret = secret.into();
        let public = match secret {
            SecretKey::Ed25519(pair) => pair.public.into(),
        };
        Self { secret, public }
    }

    // TO FIGURE OUT
    //

    // (1) weak signature
    // inputs: timestamp, IP, port
    // used for: FIND_NODE and PING messages

    // (2) strong signature
    // inputs: message 
    // used for: storage
}

// TODO: other thing in paper not yet understood...
// union/convergence of the two (for some validation)?
