//! The internal state of an [Application]


use super::application_capacities_description::ApplicationCapacitiesDescription;
use std::collections::HashMap;

pub struct ApplicationDescription {
    pub(crate) app_name: String,
    capacities: HashMap<String, ApplicationCapacitiesDescription>,
    scaleout_count: i32,
    minimum_nodes: i32,
    application_id: u64,
}
