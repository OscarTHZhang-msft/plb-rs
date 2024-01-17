//! Represents a service in a Service Fabric cluster.

use super::servicedescription::ServiceDescription;

pub struct Service {
    service_description: ServiceDescription,
}
