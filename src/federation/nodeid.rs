use crate::common::largeinteger::LargeInteger;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId {
    id_value: LargeInteger,
}

impl NodeId {
    pub fn default() -> NodeId {
        NodeId {
            id_value: LargeInteger::default(),
        }
    }

    pub fn new(id_value: LargeInteger) -> NodeId {
        NodeId { id_value: id_value }
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeId")
            .field("id_value", &self.id_value)
            .finish()
    }
}

impl Iterator for NodeId {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        Some(NodeId::new(self.id_value + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_node_id() {
        let node_id = NodeId::default();
        assert_eq!(node_id.id_value, LargeInteger::default());
    }

    #[test]
    fn test_new_node_id() {
        let large_integer = LargeInteger::new(123, 456);
        let node_id = NodeId::new(large_integer);
        assert_eq!(node_id.id_value, large_integer);
    }
}
