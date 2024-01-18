//! Represents a service in a Service Fabric cluster.

use super::service_description::ServiceDescription;

pub struct Service {
    service_description: ServiceDescription,
}
