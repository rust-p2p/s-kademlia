// Copyright 2019 4meta5 <asinghchrony@protonmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// s-kademlia DHT implementation
//
// This project aspires to provide a flexible framework for configuring
// [s/kademlia](https://www.researchgate.net/publication/4319659_SKademlia_A_practicable_approach_towards_secure_key-based_routing),
// to enhance kademlia with certain security features.
#![feature(todo_macro)]

extern crate failure;
#[macro_use] extern crate failure_derive;

mod node_id;
mod node;
mod store;
mod config;
mod error;

pub use ed25519_dalek as ed25519;

/*
* Implementation Details
* node is stored in a kbucket which is a stored in a table
* weak_signatures come from nodes
* strong signatures sign messages associated with `node_id`s (this layer of indirection seems poorly designed)
* every layer that requires async access should be configured for async access
*/
