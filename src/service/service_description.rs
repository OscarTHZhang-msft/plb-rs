//! The internal state of a [Service]

use super::service_metric::ServiceMetric;
use super::service::Service;

pub struct ServiceDescription {
    service_name: String,
    service_type_name: String,
    application_name: String,
    is_stateful: bool,
    placement_constraints: String,
    affinitized_service: String,
    aligned_affinity: bool,
    metrics: Vec<ServiceMetric>,
    default_primary_move_cost: u32,
    default_secondary_move_cost: u32,
    default_auxiliary_move_cost: u32,
    on_every_node: bool,
    allow_multiple_instances_on_node: bool,
    partition_count: i32,
    target_replica_set_size: i32,
    has_persisted_state: bool,
    service_id: u64,
    application_id: u64,
    service_instance: u64,
}
