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

mod cache;
mod bucket;
mod key;
mod store;

pub use ed25519_dalek as ed25519;
pub use x25519_dalek as x25519;

/*
surjection from `public_key` to `node_id`
WeakSignature associated with Node
StrongSignature associated with public_key
every Node has a `node_id` which has an associated `public_key`

there exists only one `public_key` for every `node_id` but 
every `public_key` may generate an infinite number of `node_id`
=> the criteria for `node_id` is designed to prevent sybil attacks
*/

/*
* node is stored in a kbucket which is a stored in a table
* weak_signatures come from nodes
* strong_signatures come from public_keys (which could have multiple associated nodes)
--> should probably restructure so both signature_types come from node (node has associated public_key and uses private_key to sign PublicSignature)
*/