use std::collections::HashMap;

use uuid::Uuid;

pub struct Replica;

pub struct FailoverUnit {
    failover_unit_description: FailoverUnitDescription,
}

impl FailoverUnit {
    pub fn id(&self) -> Uuid {
        self.failover_unit_description.id
    }
}

pub struct FailoverUnitDescription {
    pub(crate) id: Uuid,
    service_name: String,
    replicas: HashMap<Uuid, Replica>,
}
