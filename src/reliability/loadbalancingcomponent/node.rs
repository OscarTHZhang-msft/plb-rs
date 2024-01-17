use crate::federation::nodeinstance::NodeInstance;
use crate::common::nodeimagesinfo::NodeImagesInfo;
use crate::common::stopwatchtime::StopwatchTime;

use super::inbuildlimits::InBuildLimits;
use super::nodedescription::NodeDescription;

pub struct Node
{
    node_description : NodeDescription,
    node_images :  NodeImagesInfo,
    in_build_limits: InBuildLimits,
    pending_block_start_time: StopwatchTime,
}

impl Node {
    pub fn default() -> Node{
        Node{
            node_description: NodeDescription::default(),
            node_images: NodeImagesInfo::default(),
            in_build_limits: InBuildLimits::default(),
            pending_block_start_time: StopwatchTime::default(), 
        }
    }

    pub fn new(node_description: NodeDescription, node_images: NodeImagesInfo, in_build_limits: InBuildLimits, pending_block_start_time: StopwatchTime) -> Node{
        Node{
            node_description: node_description,
            node_images: node_images,
            in_build_limits: in_build_limits,
            pending_block_start_time: pending_block_start_time, 
        }
    }
}