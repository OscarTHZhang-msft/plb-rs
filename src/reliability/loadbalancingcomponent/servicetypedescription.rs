use crate::federation::nodeid::NodeId;
use std::collections::HashSet;

pub struct ServiceTypeDescription {
    name: String,
    block_list: HashSet<NodeId>,
}
