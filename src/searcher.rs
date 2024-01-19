use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::{scheduler::Phase, ClusterSnapshot};

#[derive(Debug, Clone)]
pub enum Action {
    /// Placement action. Placing a new replica on a failover unit
    NewReplicaPlacement(Vec<Uuid>),
    /// Placement action. Delete an existing replia on a failover unit
    Upgrade(Vec<Uuid>),
    // Balancing action
    LoadBalancing,
    // Balancing action
    Defragmentation,
    // ConstraintCheck action
    FixConstraintViolation,
}

pub struct Searcher {
    snapshot: Rc<RefCell<ClusterSnapshot>>,
}

impl Searcher {
    pub fn new(snapshot: &Rc<RefCell<ClusterSnapshot>>) -> Self {
        Searcher {
            snapshot: Rc::clone(snapshot),
        }
    }

    pub fn generate_actions(&self, phase: Phase) -> Vec<Action> {
        match phase {
            Phase::Placement => {
                // search for placement
                let fus_for_placement = self
                    .snapshot
                    .borrow()
                    .failover_units
                    .iter()
                    .filter_map(|(fu_id, fu)| {
                        if fu.replia_diff() > 0 {
                            Some(*fu_id)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Uuid>>();

                let fus_for_upgrade = self
                    .snapshot
                    .borrow()
                    .failover_units
                    .iter()
                    .filter_map(|(fu_id, fu)| {
                        if fu.replia_diff() < 0 {
                            Some(*fu_id)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Uuid>>();

                let mut actions = vec![];
                if !fus_for_placement.is_empty() {
                    actions.push(Action::NewReplicaPlacement(fus_for_placement));
                }
                if !fus_for_upgrade.is_empty() {
                    actions.push(Action::Upgrade(fus_for_upgrade));
                }

                actions
            }
            Phase::LoadBalancing => todo!(),
            Phase::ConstraintCheck => todo!(),
        }
    }
}
