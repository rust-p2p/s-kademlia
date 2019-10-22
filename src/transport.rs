// not meant to be complete, but necessary for basic testing as the other parts are built
// => NodeConfig, LocalDB 
// ==> "reputation" for incentives

// Node Configuration
#[derive(Debug, Clone)]
struct NodeConfig {
    // routing tables and other local storage
    // phantomdata for adding new things
    // Protocol for gossip...
}

// -- reputation --

/// see incentives work and could manage here? (open issue)
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum NodeScore<T: /*some low-friction arithmetic type for scoring (holds strategy for nodes)*/> {
    /// The node is considered connected.
    Connected(T),
    /// Node is not connected
    NotConnected(T),
} // T might represent some object with reputation-based information
