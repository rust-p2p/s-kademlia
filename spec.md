# spec

## NodeId

* `node_id` is like `libp2p/core/peer_id` but it strives to offer first-class support for anti-sybil mechanisms to slow id generation. I eventually want the proof of work method to be configurable based on some defined shared state (like the blockchain pow algorithm), but this is not my first priority.
* Instead, I'd prefer to slow joining/leaving by using governance protocols to organize nodes into sets of workers with network topology and bitswap (data transfer) protocol configured based on the nature of the assigned tasks. This will borrow heavily from the libp2p `Provider` abstraction.

## Node

* must support signing, which should follow this structure
    * `weak (timestramp, ip, port)` `=>`used for `PING` and `FIND_NODE` messages
    * `strong (message)` `=>` used for DHT storage messages
* storage will eventually be partitioned based on the nature of the stored data

## Bitswap

A generic data transfer protocol with configurable encoding (first-class support for binary encoding/decoding, not protobufs yet). This is being specified in TLA now.

## Brahms

For membership-based gossip

## Chord, UrDHT (embedding latency)

`red-blue` taught us to partition the network based on data type to provide nuanced guarantees that align with real-world requirements. `s-kademlia` may be useful for certain communication while chord with some metric for embedding latency might be useful for other task organization.



