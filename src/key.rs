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
    secret_key: &'a SecretKey<'a>,
    // associated id? for (1) and (2)
}

// define some generic public key
pub struct PublicKey(ed25519::PublicKey);

// figure out the right syntax for this based on David's code
pub struct SecretKey('a, T: &'a ed25519::Keypair);

impl<'a> Key<'a> {
    // generate a keypair
    pub fn new<T: Into<SecretKey<'a>>>(secret: T) -> Self {
        // not understanding how David's code works...
        // -- just need to generate priv, public pair for these functionalities

        let secret = secret.into();
        let public = match secret {
            SecretKey::Ed25519(pair) => pair.public.into(),
        };
        // find an assign an id using this public key
        // -- how is this verified?
        Self { secret, public }
    }
}

// TODO: other thing in paper not yet understood...
// union/convergence of the two (for some validation)?
