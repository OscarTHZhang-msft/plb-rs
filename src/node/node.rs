//! Represents a node in a Service Fabric cluster.

use super::node_description::NodeDescription;

pub struct Node {
    node_description: NodeDescription,
}
