use std::{cell::RefCell, rc::Rc};

use crate::{searcher::Action, ClusterSnapshot};

#[derive(Debug, Clone)]
pub enum Solution {
    AddReplica(String),
    DeleteReplica(String),
    MoveReplica(String),
    SwapReplica(String),
}

pub struct Solver {
    snapshot: Rc<RefCell<ClusterSnapshot>>,
}

impl Solver {
    pub fn new(snapshot: &Rc<RefCell<ClusterSnapshot>>) -> Self {
        Solver {
            snapshot: Rc::clone(snapshot),
        }
    }

    pub fn generate_solutions(&self, actions: Vec<Action>) -> Vec<Solution> {
        let mut solutions = vec![];
        for action in actions {
            match action {
                Action::NewReplicaPlacement(fu_ids) => {
                    for fu_id in fu_ids {
                        // This is a dummy PLB so we select the max node id for placement
                        if let Some((node_id, _)) = self.snapshot.borrow().nodes.last_key_value() {
                            solutions.push(Solution::AddReplica(format!(
                                "Partition {:}: AddReplica on Node {:?}",
                                fu_id, node_id
                            )));
                        }
                    }
                }
                Action::Upgrade(_) => todo!(),
                Action::LoadBalancing => todo!(),
                Action::Defragmentation => todo!(),
                Action::FixConstraintViolation => todo!(),
            }
        }

        solutions
    }
}
