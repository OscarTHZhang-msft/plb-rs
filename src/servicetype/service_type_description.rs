//! The internal state of a [ServiceType]

use super::service_type::ServiceType;
use crate::node::node_id::NodeId;
use std::collections::HashSet;

pub struct ServiceTypeDescription {
    name: String,
    block_list: HashSet<NodeId>,
}
