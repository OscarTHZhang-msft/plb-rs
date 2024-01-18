//! The internal state of an [Application]

use std::collections::HashMap;
use super::application_capacities_description::ApplicationCapacitiesDescription;
use super::application::Application;

pub struct ApplicationDescription {
    app_name: String,
    capacities: HashMap<String, ApplicationCapacitiesDescription>,
    scaleout_count: i32,
    minimum_nodes: i32,
    application_id: u64,
}
