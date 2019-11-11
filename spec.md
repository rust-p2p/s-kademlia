# spec

The DHT needs to provide the following mappings:
* `NodeId` `=>` `addr`, `port`
* `writers` `=>` `[peer_id]`
* `cid` `=>` `[peer_id]`

## NodeId

* `node_id` is like `libp2p/core/peer_id` but it strives to offer first-class support for anti-sybil mechanisms to slow id generation. I eventually want the proof of work method to be configurable based on some defined shared state (like the blockchain pow algorithm), but this is not my first priority.
* Instead, I'd prefer to slow joining/leaving by using governance protocols to organize nodes into sets of workers with network topology and bitswap (data transfer) protocol configured based on the nature of the assigned tasks. This will borrow heavily from the libp2p `Provider` abstraction.

## NodeInfo

* must support signing, which should follow this structure
    * `weak (timestramp, ip, port)` `=>`used for `PING` and `FIND_NODE` messages
    * `strong (message)` `=>` used for DHT storage messages
* storage will eventually be partitioned based on the nature of the stored data

## Store

To be abstracted into more modular storage traits
- see dynamic-sized arrays vs static routing tables
(overarching goal is to partition DHT based on data type)
- should use associated `PROVIDER` abstraction from libp2p

The **routing table** consists of a list of `N` k-buckets holding nodes with a distance `d` with `2^{i - 1} \leq d \less 2^i, 0 \leq i \leq n` and a sorted list of siblings of size `n_sigma * s`

Long-Term TODO: abstract storage containers into traits and macros like in Substrate
- vision is a network topology that adapts according to voting/gossip by nodes
- different data store for bloom filter cache for r5n
- different data store for PeerId membership via Brahms gossip

### lookup over disjoint paths 

use `d` disjoint paths to increase the lookup success ratio in a network with adversarial nodes; by using the sibling list, the lookup doesn't converge at a single node but terminates on *d* close-by neighbors, which all know the complete *s* siblings for the destination keys `=>` lookup is still successful even if `k-1` of the neighbors are adversarial

### message categorization

Categorize signalling messages to the following classes:
* incoming signed RPC requests
* responses
* unsigned messages

Each message contains the **sender address**. The sender address is *valid* if the message is signed and *actively valid*.

Actively valid sender addresses are immediately added to their corresponding bucket, when it is not full. Valid sender addresses are only added to a bucket if the *nodeId* prefix differs in an appropriate amount of bits `\psi` (`\psi > 32`).
> sender addresses that come from unsigned messages will be ignored

### Node Eviction Policy

(from the original paper) Least-recently seen eviction policy, except live nodes are never removed from the list; when a kademlia node receives any message (request or reply) from another node, it updates the appropriate k-bucket for the sender's nodeID.
- If the sending node already exists in the recipient's k-bucket and the bucket has fewer than k entries, then the recipient just inserts the new sender at the tail of the list.
- If the appropriate k-bucket is full, then the recipient pings the k-bucket's least recently seen node.
- If it fails to respond, it's evicted and new node is inserted
- else (if it responds), the least recently seen node is moved to the tail of the list, and the new sender's contact is discarded

## NodeHandler

Eventually, I'd like to partition storage according to the type of data communicated over the network.

**fields I'm considering adding**
* block_store (like `bitswap::bddb`)
* use `sled::db`
* libp2p `Provider` abstraction for pull response interface
* `Arc<Protocol>`

Signing Trait for NodeConfig with
* `WeakSignature`
* `StrongSignature`

Node Discovery
* iterative, disjoint lookup paths that converge upon the same path

## Brahms

For membership-oriented gossip

## Chord, UrDHT (embedding latency)

`red-blue` taught us to partition the network based on data type to provide nuanced guarantees that align with real-world requirements. `s-kademlia` may be useful for certain communication while chord with some metric for embedding latency might be useful for other task organization.

## reliable sibling broadcast

Common security problem is the reliability of sibling information which arises when replicated information needs to be stored in the DHT which uses a majority decision to compensate for adversarial nodes.
> see [10](http://www.cs.kent.edu/~javed/class-IAD06S/papers-2004/gai.pdf) for definition of a *sibling* list to manage certain lists of `(id, value)` pairs
