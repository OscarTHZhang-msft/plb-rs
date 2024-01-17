use std::fmt;
use crate::common::largeinteger::LargeInteger;

#[derive(Copy, Clone)]
pub struct NodeId
{
    id_value : LargeInteger, 
}

impl NodeId {
    pub fn default() -> NodeId{
        NodeId{
            id_value: LargeInteger::default(),
        }
    }

    pub fn new(id_value: LargeInteger) -> NodeId{
        NodeId{
            id_value: id_value,
        }
    }
}

impl PartialEq for NodeId {
    fn eq(&self, other: &Self) -> bool {
        self.id_value == other.id_value
    }
}

impl Eq for NodeId {}

impl fmt::Debug for NodeId
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeId")
         .field("id_value", &self.id_value)
         .finish()
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
