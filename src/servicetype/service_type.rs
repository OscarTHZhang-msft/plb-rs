//! Represents a service type in a Service Fabric cluster.

use super::service_type_description::ServiceTypeDescription;

pub struct ServiceType {
    pub(crate) service_type_desc: ServiceTypeDescription,
}

impl ServiceType {
    pub fn service_type_name(&self) -> &str {
        &self.service_type_desc.name
    }
}
