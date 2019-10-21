use crate::id::{NodeTableId, NodeId};
use crate::node::Node;

/// K-bucket - structure for keeping last nodes in Kademlia.
pub struct KBucket<IPAddr, Port, NodeId> {
    // VecDeque vs ArrayVec? open issue
    data: VecDeque<Node<IPAddr, Port, NodeId>>,
    size: usize, // why usize? \exists default
}

// issue
// - building a more general configuration (can be done by using the parameters of the KBucket)

impl<IPAddr, Port, NodeId> KBucket<IPAddr, Port, NodeId>
where
    IpAddr: Clone + Debug, // TODO: specify trait bounds in context of `node::WeakSignature` as the container object
    Port: Clone + Debug + Send,
{
    pub fn new(k: usize) -> KBucket<IPAddr, Port, NodeId> {
        // TODO: some check here that the size fits some bound
        KBucket {
            data: VecDeque::new(),
            size: k,
        }
    }

    pub fn update(&mut self, node: &Node<IPAddr, Port, NodeId>) -> bool {
        if self.data.iter().any(|x| x.id == node.id) {
            self.update_position(node.clone());
            debug!("Promoted node {:?} to the top of kbucket", node);
            true
        } else if self.data.len() == self.size {
            debug!("Not adding new node {:?} to kbucket - no space left", node);
            false
        } else {
            self.data.push_back(node.clone());
            debug!("Added new node {:?} to kbucket", node);
            true
        }
    }

    pub fn find(&self, id: &NodeId, count: usize) -> Vec<Node<IPAddr, Port, NodeId>> {
        let mut data_copy: Vec<_> = self.data.iter().map(|n| n.clone()).collect();
        data_copy.sort_by_key(|n| KNodeTable::<TId, TAddr>::distance(id, &n.id));
        data_copy[0..cmp::min(count, data_copy.len())].to_vec()
    }

    pub fn data(&self) -> &VecDeque<Node<TId, TAddr>> {
        &self.data
    }
    pub fn size(&self) -> usize {
        self.size
    }

    fn update_position(&mut self, node: Node<TId, TAddr>) {
        let mut new_data = VecDeque::with_capacity(self.data.len());
        new_data.extend(
            self.data
                .iter()
                .filter(|x| x.id != node.id)
                .map(|x| x.clone()),
        );
        new_data.push_back(node.clone());
        self.data = new_data;
    }
}