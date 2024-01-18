//! Represents a service type in a Service Fabric cluster.

use super::service_type_description::ServiceTypeDescription;

pub struct ServiceType {
    service_type_desc: ServiceTypeDescription,
}
