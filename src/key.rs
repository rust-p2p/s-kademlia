use crate::ed25519;
use crate::error;
// use time::SteadyTime;
use zeroize;

// TODO:
// - make this generic over multiple key types instead of just ed25519
pub struct KeyPair(ed25519::KeyPair);

impl KeyPair {
    pub fn new() -> KeyPair {
        // anti-sybil research pursued under issue #3
        KeyPair(ed25519::Keypair::generate(&mut rand::thread_rng()))
    }

    fn encode(&self) -> [u8; 64] {
        self.0.to_bytes()
    }

    pub fn decode(kp: &mut [u8]) -> Result<KeyPair, DecodingError> {
        ed25519::Keypair::from_bytes(kp)
            .map(|k| {
                kp.zeroize();
                Keypair(k)
            })
            .map_err(|e| DecodingError::new("Ed25519 keypair").source(e.compat()))
    }

    pub fn sign(&self, msg: &[u8]) -> Vec<u8> {
        self.0.sign(msg).to_bytes().to_vec()
    }

    pub fn public(&self) -> PublicKey {
        PublicKey(self.0.public)
    }

    pub fn secret(&self) -> SecretKey {
        SecretKey::from_bytes(&mut self.0.secret.to_bytes())
            .expect("ed25519::SecretKey::from_bytes(to_bytes(k)) != k")
    }
}

impl Clone for KeyPair {
    fn clone(&self) -> KeyPair {
        let mut sk_bytes = self.0.secret.to_bytes();
        let secret = SecretKey::from_bytes(&mut sk_bytes)
            .expect("ed25519::SecretKey::from_bytes(to_bytes(k)) != k")
            .0;
        let public = ed25519::PublicKey::from_bytes(&self.0.public.to_bytes())
            .expect("ed25519::PublicKey::from_bytes(to_bytes(k)) != k");
        KeyPair(ed25519::Keypair { secret, public })
    }
}

impl From<Keypair> for SecretKey {
    fn from(kp: Keypair) -> SecretKey {
        SecretKey(kp.0.secret)
    }
}

impl From<SecretKey> for Keypair {
    fn from(sk: SecretKey) -> Keypair {
        let secret = sk.0;
        let public = ed25519::PublicKey::from(&secret);
        Keypair(ed25519::Keypair { secret, public })
    }
}
