//! Represents a service type in a Service Fabric cluster.

use super::servicetypedescription::ServiceTypeDescription;

pub struct ServiceType {
    service_type_desc: ServiceTypeDescription,
}
