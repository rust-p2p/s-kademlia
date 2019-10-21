use std::error::Error; // TODO: use alongside `failure`
use std::fmt;

/// Errors for this s-kademlia implementation
/// TODO: the code for the impls is basically the same
/// -- could use a macro instead of this code duplication...

/// An error during encoding of key material.
#[derive(Debug)]
pub struct DecodingError {
    msg: String,
    source: Option<Box<dyn Error + Send + Sync>>
}

impl DecodingError {
    pub(crate) fn new<S: ToString>(msg: S) -> Self {
        Self { msg: msg.to_string(), source: None }
    }

    pub(crate) fn source(self, source: impl Error + Send + Sync + 'static) -> Self {
        Self { source: Some(Box::new(source)), .. self }
    }
}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key decoding error: {}", self.msg)
    }
}

impl Error for DecodingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|s| &**s as &dyn Error)
    }
}

/// An error using anti-sybil band-aid while generating public keys
#[derive(Debug)]
pub struct AntiSybilError {
    msg: String,
    // -- might be useful once more work is done on this mechanism
    // source: Option<Box<dyn Error + Send + Sync>>
}

impl AntiSybilError {
        pub(crate) fn new<S: ToString>(msg: S) -> Self {
        Self { msg: msg.to_string()/*, source: None*/ }
    }

    // pub(crate) fn source(self, source: impl Error + Send + Sync + 'static) -> Self {
    //     Self { source: Some(Box::new(source)), .. self }
    // }
}

impl fmt::Display for AntiSybilError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Anti-sybil mechanism timeout: {}", self.msg)
    }
}

// impl Error for AntiSybilError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         self.source.as_ref().map(|s| &**s as &dyn Error)
//     }
// }

/// An error during signing of a message.
#[derive(Debug)]
pub struct SigningError {
    msg: String,
    source: Option<Box<dyn Error + Send + Sync>>
}
.
impl SigningError {
    pub(crate) fn new<S: ToString>(msg: S) -> Self {
        Self { msg: msg.to_string(), source: None }
    }

    pub(crate) fn source(self, source: impl Error + Send + Sync + 'static) -> Self {
        Self { source: Some(Box::new(source)), .. self }
    }
}

impl fmt::Display for SigningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key signing error: {}", self.msg)
    }
}

impl Error for SigningError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|s| &**s as &dyn Error)
    }
}

