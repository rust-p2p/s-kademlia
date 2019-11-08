use bs58;
pub use failure::Error;
use std::{convert::From, fmt};

#[derive(Debug, Fail)]
#[fail(display = "parsing via bs58 did not succeed")]
pub struct ParseError;
// TODO: consider splitting into enum with
// (1) length-specific error
// (2) decoding error

impl From<bs58::decode::Error> for ParseError {
    fn from(error: bs58::decode::Error) -> Self {
        ParseError
    }
}

// NOTE: caveats on this pattern https://boats.gitlab.io/failure/custom-fail.html
// (poor forward compatibility)
#[derive(Debug, Fail)]
pub enum NodeIdGenError {
    #[fail(display = "public key hash yielded zero byte array")]
    PubkeyHashZero,
    #[fail(display = "anti sybil generation timed out")]
    HardGenTimeOut,
}

// TODO
// - SigningError
