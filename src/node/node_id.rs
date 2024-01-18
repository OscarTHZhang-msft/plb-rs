//! NodeId is a unique identifier for a node in the cluster.

use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId {
    id_value: u128,
}

impl NodeId {
    pub fn default() -> NodeId {
        NodeId {
            id_value: u128::default(),
        }
    }

    pub fn new(id_value: u128) -> NodeId {
        NodeId { id_value: id_value }
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeId")
            .field("id_value", &self.id_value)
            .finish()
    }
}

impl Iterator for NodeId {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        Some(NodeId::new(self.id_value + 1))
    }
}
