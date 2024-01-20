use std::collections::HashMap;

use uuid::Uuid;

use crate::node::{node_id::NodeId};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum ReplicaRole {
    None = 1024,
    Primary = 1025,
    Secondary = 1026,
    StandBy = 1027,
    Dropped = 1028,
    Auxiliary = 1029,
    StandByAuxiliary = 1030,
}

#[derive(Debug, Clone)]
pub struct Replica {
    replica_id: u128,
    fu_id: Uuid,
    role: ReplicaRole,
    location: NodeId,
}

impl Replica {
    pub fn new(replica_id: u128, fu_id: Uuid, role: ReplicaRole, location: NodeId) -> Self {
        Replica {
            replica_id,
            fu_id,
            role,
            location,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FailoverUnit {
    pub(crate) failover_unit_description: FailoverUnitDescription,
}

impl FailoverUnit {
    pub fn id(&self) -> Uuid {
        self.failover_unit_description.id
    }

    pub fn replia_diff(&self) -> i32 {
        self.failover_unit_description.replica_diff
    }
}

#[derive(Debug, Clone, Default)]
pub struct FailoverUnitDescription {
    pub(crate) id: Uuid,
    pub(crate) service_name: String,
    pub(crate) replicas: HashMap<Uuid, Replica>,
    pub(crate) replica_diff: i32,
}
