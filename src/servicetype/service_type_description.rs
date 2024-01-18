//! The internal state of a [ServiceType]

use std::collections::HashSet;
use crate::node::node_id::NodeId;
use super::service_type::ServiceType;

pub struct ServiceTypeDescription {
    name: String,
    block_list: HashSet<NodeId>,
}
