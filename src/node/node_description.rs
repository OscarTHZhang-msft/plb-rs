//! The internal state of a [Node]



use super::node_instance::NodeInstance;
use std::collections::HashMap;

type DomainId = String;

pub struct NodeDescription {
    pub(crate) node_instance: NodeInstance,
    pub(crate) is_up: bool,
    pub(crate) capacity_ratios: HashMap<String, u32>,
    pub(crate) capacities: HashMap<String, u32>,
}

impl NodeDescription {
    pub fn default() -> NodeDescription {
        NodeDescription {
            node_instance: NodeInstance::default(),
            is_up: true,
            capacity_ratios: HashMap::new(),
            capacities: HashMap::new(),
        }
    }

    pub fn new(
        node_instance: NodeInstance,
        is_up: bool,
        capacity_ratios: HashMap<String, u32>,
        capacities: HashMap<String, u32>,
    ) -> NodeDescription {
        NodeDescription {
            node_instance,
            is_up,
            capacity_ratios,
            capacities,
        }
    }
}
