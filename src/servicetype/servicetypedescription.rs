//! The internal state of a [ServiceType]

use std::collections::HashSet;
use crate::node::nodeid::NodeId;
use super::servicetype::ServiceType;

pub struct ServiceTypeDescription {
    name: String,
    block_list: HashSet<NodeId>,
}
