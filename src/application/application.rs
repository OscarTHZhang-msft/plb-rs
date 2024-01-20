//! Represents an application in a Service Fabric cluster.

use super::application_description::ApplicationDescription;
use std::collections::HashSet;

pub struct Application {
    pub(crate) application_desc: ApplicationDescription,
    pub(crate) services: HashSet<String>,
}

impl Application {
    pub fn app_name(&self) -> &str {
        &self.application_desc.app_name
    }
}
