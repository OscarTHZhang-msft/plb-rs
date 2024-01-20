//! Represents a service in a Service Fabric cluster.

use super::service_description::ServiceDescription;

pub struct Service {
    pub(crate) service_description: ServiceDescription,
}

impl Service {
    pub fn servcie_name(&self) -> &str {
        &self.service_description.service_name
    }
}
