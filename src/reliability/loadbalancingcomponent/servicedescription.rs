use super::servicemetric::ServiceMetric;
use super::servicescalingindexgroup::ServiceScalingIndexGroup;
use super::servicesensitivity::ServiceSensitivity;
use crate::reliability::scalingmechanism::ScalingMechanism;
use crate::reliability::scalingmechanismkind::ScalingMechanismKind;
use crate::reliability::servicescalingpolicydescription::ServiceScalingPolicyDescription;
use crate::reliability::servicetagscollection::ServiceTagsCollection;
use crate::servicemodel::servicepackageactivationmode::ServicePackageActivationMode;
use crate::servicemodel::servicepackageidentifier::ServicePackageIdentifier;

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
    service_package_id: u64,
    service_package_identifier: ServicePackageIdentifier,
    service_package_activation_mode: ServicePackageActivationMode,
    service_instance: u64,

    // Auto-scaling fields
    scaling_policies: Vec<ServiceScalingPolicyDescription>,
    scaling_index_group: ServiceScalingIndexGroup,

    // If this config is defined on a service level use that value.
    // Otherwise, use the cluster level parameter value.
    is_singleton_replica_move_allowed_during_upgrade: bool, // std::shared_ptr<bool> isSingletonReplicaMoveAllowedDuringUpgrade_;

    service_tags: ServiceTagsCollection,

    default_sensitivity: ServiceSensitivity,
}
