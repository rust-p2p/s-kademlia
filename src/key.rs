/// two types of keys...

use multihash::Multihash;
use std::hash::{Hash, Hasher};

pub trait Key<T> {
    type Preimage: T;

    pub fn new(Self::Preimage) -> Self<T>;

    pub fn preimage(&self) -> &Self::Preimage;

    pub fn into_preimage(self) -> Self::Preimage;

    pub fn distance
}

// weak key

// strong key

// consider the union of the two (w/ validation)?
