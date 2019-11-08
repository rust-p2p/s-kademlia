pub use failure::Error;
use std::{fmt, convert::From};
use bs58;

#[derive(Debug, Fail)]
#[fail(display = "parsing via bs58 did not succeed")]
pub struct ParseError;

impl From<bs58::decode::Error> for ParseError {
    fn from(error: bs58::decode::Error) -> Self {
        ParseError
    }
}

#[derive(Debug, Fail)]
pub enum NodeIdGenError {
    #[fail(display = "public key hash yielded zero byte array")]
    PubkeyHashZero,
    #[fail(display = "anti sybil generation timed out")]
    HardGenTimeOut,
}

// TODO
// - LengthDisparityError
// - SigningError
