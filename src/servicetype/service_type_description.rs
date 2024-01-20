//! The internal state of a [ServiceType]


use crate::node::node_id::NodeId;
use std::collections::HashSet;

#[derive(Default)]
pub struct ServiceTypeDescription {
    pub(crate) name: String,
    block_list: HashSet<NodeId>,
}
