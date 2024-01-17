use std::collections::HashMap;

use crate::federation::nodeinstance::NodeInstance;
use crate::reliability::nodedeactivationintent::NodeDeactivationIntent;
use crate::reliability::nodedeactivationstatus::NodeDeactivationStatus;

type DomainId = String;

pub struct NodeDescription {
    node_instance: NodeInstance,
    is_up: bool,
    deactivation_intent: NodeDeactivationIntent,
    deactivation_status: NodeDeactivationStatus,
    node_properties: HashMap<String, String>,
    fault_domain_id: DomainId,
    upgrade_domain_id: String,
    capacity_ratios: HashMap<String, u32>,
    capacities: HashMap<String, u32>,
    is_pending_close: bool,
    is_replica_uploaded: bool,
}

impl NodeDescription {
    pub fn default() -> NodeDescription {
        NodeDescription {
            node_instance: NodeInstance::default(),
            is_up: false,
            deactivation_intent: NodeDeactivationIntent::None,
            deactivation_status: NodeDeactivationStatus::None,
            node_properties: HashMap::new(),
            fault_domain_id: String::new(),
            upgrade_domain_id: String::new(),
            capacity_ratios: HashMap::new(),
            capacities: HashMap::new(),
            is_pending_close: false,
            is_replica_uploaded: false,
        }
    }

    pub fn new(
        node_instance: NodeInstance,
        is_up: bool,
        deactivation_intent: NodeDeactivationIntent,
        deactivation_status: NodeDeactivationStatus,
        node_properties: HashMap<String, String>,
        fault_domain_id: DomainId,
        upgrade_domain_id: String,
        capacity_ratios: HashMap<String, u32>,
        capacities: HashMap<String, u32>,
        is_pending_close: bool,
        is_replica_uploaded: bool,
    ) -> NodeDescription {
        NodeDescription {
            node_instance,
            is_up,
            deactivation_intent,
            deactivation_status,
            node_properties,
            fault_domain_id,
            upgrade_domain_id,
            capacity_ratios,
            capacities,
            is_pending_close,
            is_replica_uploaded,
        }
    }
}
