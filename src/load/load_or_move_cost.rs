use uuid::Uuid;

pub struct LoadOrMoveCost {
    load_description: LoadOrMoveCostDescription,
}

impl LoadOrMoveCost {
    pub fn id(&self) -> Uuid {
        self.load_description.fu_id
    }
}

pub struct LoadOrMoveCostDescription {
    pub(crate) fu_id: Uuid,
}
