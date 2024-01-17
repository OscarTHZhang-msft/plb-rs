use crate::federation::nodeinstance::NodeInstance;

pub struct InBuildLimits{
    node_instance : NodeInstance,
    global_in_build_limit : i32,
    placement_in_build_limit : i32,
    constraint_check_in_build_limit : i32,
    balancing_in_build_limit : i32,
}

impl InBuildLimits {
    pub fn default() -> InBuildLimits{
        InBuildLimits{
            node_instance: NodeInstance::default(),
            global_in_build_limit: 0,
            placement_in_build_limit: 0,
            constraint_check_in_build_limit: 0,
            balancing_in_build_limit: 0,
        }
    }

    pub fn new(node_instance: NodeInstance, global_in_build_limit: i32, placement_in_build_limit: i32, constraint_check_in_build_limit: i32, balancing_in_build_limit: i32) -> InBuildLimits{
        InBuildLimits{
            node_instance: node_instance,
            global_in_build_limit: global_in_build_limit,
            placement_in_build_limit: placement_in_build_limit,
            constraint_check_in_build_limit: constraint_check_in_build_limit,
            balancing_in_build_limit: balancing_in_build_limit,
        }
    }
}
