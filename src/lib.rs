// Copyright 2019 Amar "4meta5" Singh <asinghchrony@protonmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//

//! s-kademlia DHT implementation
//!
//! This project aspires to provide a flexible framework for configuring
//! [s/kademlia](https://www.researchgate.net/publication/4319659_SKademlia_A_practicable_approach_towards_secure_key-based_routing),
//! to enhance kademlia with certain security features.

mod error;
mod id;
mod key;
mod node;
mod store;

pub use ed25519_dalek as ed25519;
pub use x25519_dalek as x25519;

/*
Implementation Details
* node is stored in a kbucket which is a stored in a table
* weak_signatures come from nodes
* strong signatures sign messages associated with `node_id`s (this layer of indirection seems poorly designed)
* every layer that requires async access should be configured for async access
*/

/*
Managing the mapping between id generation and public key seems like a common problem
- I propose no anti-sybil mechanism, but make it pluggable (have a function in the file and comment it out? add benches for it)
*/
