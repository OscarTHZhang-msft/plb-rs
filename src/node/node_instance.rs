use super::node_id::NodeId;

pub struct NodeInstance {
    id: NodeId,
    instance_id: u64,
}

impl NodeInstance {
    pub fn default() -> NodeInstance {
        NodeInstance {
            id: NodeId::default(),
            instance_id: 0,
        }
    }

    pub fn new(id: NodeId, instance_id: u64) -> NodeInstance {
        NodeInstance {
            id: id,
            instance_id: instance_id,
        }
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
        let node_id = NodeId::new(LargeInteger::new(123, 456));
        let instance_id = 123;
        let node_instance = NodeInstance::new(node_id, instance_id);
        assert_eq!(node_instance.id, node_id);
        assert_eq!(node_instance.instance_id, instance_id);
    }
}
