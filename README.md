# s/kademlia

pure Rust implementation of [s/kademlia](https://www.researchgate.net/publication/4319659_SKademlia_A_practicable_approach_towards_secure_key-based_routing)

## why

prevents sybil attacks on the address space by creating a minimum work threshold for node generation (storage NodeId generation requires *trailing* bits of 0s `=>` slows down process of adding new nodes)

## implementation details