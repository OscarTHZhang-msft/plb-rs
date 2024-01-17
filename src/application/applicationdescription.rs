use super::applicationcapacitiesdescription::ApplicationCapacitiesDescription;
use std::collections::HashMap;

pub struct ApplicationDescription {
    app_name: String,
    capacities: HashMap<String, ApplicationCapacitiesDescription>,
    scaleout_count: i32,
    minimum_nodes: i32,
    application_id: u64,
}
