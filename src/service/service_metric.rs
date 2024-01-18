use super::built_in_type::BuiltInType;

pub struct ServiceMetric {
    name: String,
    built_in_type: BuiltInType,
    weight: f64,
    primary_default_load: u32,
    secondary_default_load: u32,
    auxilliary_default_load: u32,
    maxium_load: u32,
    is_rg_metric: bool,
}
