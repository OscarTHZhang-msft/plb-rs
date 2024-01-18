//! Represents an application in a Service Fabric cluster.

use super::application_description::ApplicationDescription;
use std::collections::HashSet;

pub struct Application {
    application_desc: ApplicationDescription,
    services: HashSet<String>,
}
