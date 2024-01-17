use std::collections::HashMap;

use super::nodeinstance::NodeInstance;

type DomainId = String;

pub struct NodeDescription {
    node_instance: NodeInstance,
    is_up: bool,
    capacity_ratios: HashMap<String, u32>,
    capacities: HashMap<String, u32>,
}

impl NodeDescription {
    pub fn default() -> NodeDescription {
        NodeDescription {
            node_instance: NodeInstance::default(),
            is_up: false,
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
