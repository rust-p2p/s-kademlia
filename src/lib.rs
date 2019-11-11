// Copyright 2019 4meta5 <asinghchrony@protonmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// S-Kademlia DHT implementation
//
// This project aspires to provide a flexible framework for configuring
// [s/kademlia](https://www.researchgate.net/publication/4319659_SKademlia_A_practicable_approach_towards_secure_key-based_routing),
// to enhance kademlia with certain security features.
#![feature(todo_macro)]

// testing and error handling
mod error;
mod util;

// in order of containment (node_id \in node \in store \in config)
mod config;
mod node;
mod node_id;
mod store;

pub use node_id::NodeId;

pub use ed25519_dalek as ed25519;
