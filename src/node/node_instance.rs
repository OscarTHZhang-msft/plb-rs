use super::node_id::NodeId;
#[derive(Debug, Clone, Copy)]
pub struct NodeInstance {
    pub(crate) id: NodeId,
    pub(crate) instance_id: u64,
}

impl Default for NodeInstance {
    fn default() -> NodeInstance {
        NodeInstance {
            id: NodeId::default(),
            instance_id: 0,
        }
    }
}

impl NodeInstance {
    pub fn new(id: NodeId, instance_id: u64) -> NodeInstance {
        NodeInstance { id, instance_id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let node_instance = NodeInstance::default();
        assert_eq!(node_instance.id, NodeId::default());
        assert_eq!(node_instance.instance_id, 0);
    }

    #[test]
    fn test_new() {
        let node_id = NodeId::new(456);
        let instance_id = 123;
        let node_instance = NodeInstance::new(node_id, instance_id);
        assert_eq!(node_instance.id, node_id);
        assert_eq!(node_instance.instance_id, instance_id);
    }
}
