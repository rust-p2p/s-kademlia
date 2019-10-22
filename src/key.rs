use crate::ed25519;
use crate::error;
// use time::SteadyTime;
use zeroize;

pub struct KeyPair(ed25519::KeyPair);

impl KeyPair {
    pub fn new() -> Result<Keypair, AntiSybilError> {

        // if pow/anti-sybil mechanism here: // see issue #3
        // comment next line out
        return Ok(ed25519::Keypair::generate(&mut rand::thread_rng()));

        // issue #3, dumb proof of work for anti-sybil
        // comment *in* these lines
        // let super_counter = SteadyTime::now()
        // loop {
        //     let new_keypair = ed25519::Keypair::generate(&mut rand::thread_rng());
        //     let difficulty: u32 = 5; // number of required trailing zeros
        //     let counter = 0u32;
        //     let success = true;
        //     new_keypair.to_bytes().into_iter().rev().for_each(|i| {
        //         if counter < difficulty && i != 0 {
        //             success = false;
        //         }
        //         counter += 1;
        //     });
        //     if success { return Ok(Keypair(new_keypair)) };
        //     super_counter += 1;
        //     if super_counter > timeout {
        //         return Err(AntiSybilError::new("Anti-sybil mechanism timed out"))
        //     }
        // };
    }

    fn encode(&self) -> [u8; 64] {
        self.0.to_bytes()
    }

    pub fn decode(kp: &mut [u8]) -> Result<Keypair, DecodingError> {
        ed25519::Keypair::from_bytes(kp)
            .map(|k| { kp.zeroize(); Keypair(k) })
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

impl Clone for Keypair {
    fn clone(&self) -> Keypair {
        let mut sk_bytes = self.0.secret.to_bytes();
        let secret = SecretKey::from_bytes(&mut sk_bytes)
            .expect("ed25519::SecretKey::from_bytes(to_bytes(k)) != k").0;
        let public = ed25519::PublicKey::from_bytes(&self.0.public.to_bytes())
            .expect("ed25519::PublicKey::from_bytes(to_bytes(k)) != k");
        Keypair(ed25519::Keypair { secret, public })
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