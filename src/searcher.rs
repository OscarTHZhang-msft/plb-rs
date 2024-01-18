use crate::scheduler::Phase;

#[derive(Debug, Clone)]
pub enum Action {
    // Placement action
    NewReplicaPlacement,
    // Placement action
    Update,
    // Balancing action
    LoadBalancing,
    // Balancing action
    Defragmentation,
    // ConstraintCheck action
    FixConstraintViolation,
}

pub struct Searcher;

impl Searcher {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn generate_actions(&self, phase: Phase) -> Vec<Action> {
        unimplemented!()
    }
}
