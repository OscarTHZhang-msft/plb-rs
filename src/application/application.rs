//! Represents an application in a Service Fabric cluster.

use super::applicationdescription::ApplicationDescription;
use std::collections::HashSet;

pub struct Application {
    application_desc: ApplicationDescription,
    services: HashSet<String>,
}
