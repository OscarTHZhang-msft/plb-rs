//! Represents a node in a Service Fabric cluster.

use super::{node_description::NodeDescription, node_id::NodeId};

pub struct Node {
    pub(crate) node_description: NodeDescription,
}

impl Node {
    pub fn node_id(&self) -> NodeId {
        self.node_description.node_instance.id
    }
}
