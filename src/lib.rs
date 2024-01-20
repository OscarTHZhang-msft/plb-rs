use std::{
    cell::RefCell,
    collections::{BTreeMap, HashSet, VecDeque},
    rc::Rc,
    sync::{Arc, Mutex},
};

pub mod application;
pub mod failoverunit;
pub mod load;
pub mod node;
pub mod scheduler;
pub mod searcher;
pub mod service;
pub mod servicetype;
pub mod solver;

use application::{application::Application, application_description::ApplicationDescription};
use failoverunit::failover_unit::{FailoverUnit, FailoverUnitDescription};
use load::load_or_move_cost::{LoadOrMoveCost, LoadOrMoveCostDescription};
use node::node_id::NodeId;
use node::{node::Node, node_description::NodeDescription};
use scheduler::PLBScheduler;
use service::{service::Service, service_description::ServiceDescription};
use servicetype::{service_type::ServiceType, service_type_description::ServiceTypeDescription};

use std::cmp::Ordering;

use anyhow::Result;
use searcher::Searcher;
use solver::{Solution, Solver};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Default)]
struct UpdateQueue {
    node_update_queue: VecDeque<Node>,
    app_update_queue: VecDeque<Application>,
    service_type_update_queue: VecDeque<ServiceType>,
    service_update_queue: VecDeque<Service>,
    failover_unit_update_queue: VecDeque<FailoverUnit>,
    load_update_queue: VecDeque<LoadOrMoveCost>,
}

#[derive(Default)]
pub struct ClusterSnapshot {
    nodes: BTreeMap<NodeId, Node>,
    apps: BTreeMap<String, Application>,
    service_types: BTreeMap<String, ServiceType>,
    services: BTreeMap<String, Service>,
    failover_units: BTreeMap<Uuid, FailoverUnit>,
    loads: BTreeMap<Uuid, LoadOrMoveCost>,
}

/// Similar to the C++ implementation. This is the main entry point of the entire PLB engine.
/// It consists of all the required data structures to basically does 3 things:
///     1. Listen to update cluster info API calls
///     2. Run the PLB refresh loop
///     3. Schedule searches and solutions if required

#[derive(Default)]
pub struct PlacementAndLoadBalancing {
    /// PLB takes snapshot of the cluster information before refreshing
    /// During the PLB refresh this cluster snapshot can not be modified
    cluster_snapshot: Rc<RefCell<ClusterSnapshot>>,
    /// PLB cannot operate on stale operation, but also cannot be interrupted by the new update when searching for solutions
    /// This update queue is guarded by a single mutex and is read by PLB on the start of the refresh
    plb_update_queue: Arc<Mutex<UpdateQueue>>,
    scheduler: PLBScheduler,
    searcher: Searcher,
    solver: Solver,
}

impl PlacementAndLoadBalancing {
    /// Initializes the PlacementAndLoadBalancing object with the required cluster information:
    ///     - Nodes
    ///     - Applications
    ///     - Service types
    ///     - Services
    ///     - Failover units
    ///     - Loads or move costs
    pub fn new(
        nodes: Vec<NodeDescription>,
        apps: Vec<ApplicationDescription>,
        service_types: Vec<ServiceTypeDescription>,
        services: Vec<ServiceDescription>,
        failover_units: Vec<FailoverUnitDescription>,
        loads: Vec<LoadOrMoveCostDescription>,
    ) -> Self {
        // copy the cluster information to cluster snapshot, without going through the update queue
        let node_map = nodes
            .into_iter()
            .map(|node_desc| {
                (
                    node_desc.node_instance.id,
                    Node {
                        node_description: node_desc,
                    },
                )
            })
            .collect::<BTreeMap<NodeId, Node>>();

        let app_map = apps
            .into_iter()
            .map(|app_desc| {
                (
                    app_desc.app_name.clone(),
                    Application {
                        application_desc: app_desc,
                        services: HashSet::new(),
                    },
                )
            })
            .collect::<BTreeMap<String, Application>>();

        let service_type_map = service_types
            .into_iter()
            .map(|service_type_desc| {
                (
                    service_type_desc.name.clone(),
                    ServiceType { service_type_desc },
                )
            })
            .collect::<BTreeMap<String, ServiceType>>();

        let service_map = services
            .into_iter()
            .map(|service_desc| {
                (
                    service_desc.service_name.clone(),
                    Service {
                        service_description: service_desc,
                    },
                )
            })
            .collect::<BTreeMap<String, Service>>();

        let fu_map = failover_units
            .into_iter()
            .map(|fu_desc| {
                (
                    fu_desc.id,
                    FailoverUnit {
                        failover_unit_description: fu_desc,
                    },
                )
            })
            .collect::<BTreeMap<Uuid, FailoverUnit>>();

        let load_map = loads
            .into_iter()
            .map(|load_desc| {
                (
                    load_desc.fu_id,
                    LoadOrMoveCost {
                        load_description: load_desc,
                    },
                )
            })
            .collect::<BTreeMap<Uuid, LoadOrMoveCost>>();

        PlacementAndLoadBalancing {
            cluster_snapshot: Rc::new(RefCell::new(ClusterSnapshot {
                nodes: node_map,
                apps: app_map,
                service_types: service_type_map,
                services: service_map,
                failover_units: fu_map,
                loads: load_map,
            })),
            plb_update_queue: Arc::new(Mutex::new(UpdateQueue::default())),
            scheduler: PLBScheduler::new(OffsetDateTime::now_utc()),
            searcher: Searcher::default(),
            solver: Solver::default(),
        }
    }

    pub fn update_node(&mut self, node_desc: NodeDescription) {
        let update_queue_clone = Arc::clone(&self.plb_update_queue);
        update_queue_clone
            .lock()
            .unwrap()
            .node_update_queue
            .push_back(Node {
                node_description: node_desc,
            });
    }

    pub fn delete_node(&mut self, _node_id: NodeId) -> Result<()> {
        unimplemented!()
    }

    pub fn update_application(&self, app_desc: ApplicationDescription) {
        let update_queue_clone = Arc::clone(&self.plb_update_queue);
        update_queue_clone
            .lock()
            .unwrap()
            .app_update_queue
            .push_back(Application {
                application_desc: app_desc,
                services: HashSet::new(),
            });
    }

    pub fn delete_application(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub fn update_service_type(&mut self, service_type_desc: ServiceTypeDescription) {
        let update_queue_clone = Arc::clone(&self.plb_update_queue);
        update_queue_clone
            .lock()
            .unwrap()
            .service_type_update_queue
            .push_back(ServiceType {
                service_type_desc,
            });
    }

    pub fn delete_service_type(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub fn update_service(&mut self, service_desc: ServiceDescription) {
        let update_queue_clone = Arc::clone(&self.plb_update_queue);
        update_queue_clone
            .lock()
            .unwrap()
            .service_update_queue
            .push_back(Service {
                service_description: service_desc,
            });
    }

    pub fn delete_service(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub fn update_failover_unit(&mut self, fu_desc: FailoverUnitDescription) {
        let update_queue_clone = Arc::clone(&self.plb_update_queue);
        update_queue_clone
            .lock()
            .unwrap()
            .failover_unit_update_queue
            .push_back(FailoverUnit {
                failover_unit_description: fu_desc,
            });
    }

    pub fn delete_failover_unit(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub fn update_load_or_move_cost(&mut self, load_desc: LoadOrMoveCostDescription) {
        let update_queue_clone = Arc::clone(&self.plb_update_queue);
        update_queue_clone
            .lock()
            .unwrap()
            .load_update_queue
            .push_back(LoadOrMoveCost {
                load_description: load_desc,
            });
    }

    /// Refresh the PLB data structures from the pending update queues.
    /// It also triggers PLBSchedular to schedule any searcher stages if any stages are due at the current timestamp of the refresh (now)
    pub fn refresh(&mut self, now: OffsetDateTime) -> Result<Vec<Solution>> {
        // Update PLB internal data structures to sync with the latest cluster information
        {
            let update_queue_clone = Arc::clone(&self.plb_update_queue);
            let mut update_queue = update_queue_clone.lock().unwrap();

            // Copy over the updates to the PLB structure for snapshot
            self.process_node_updates(&mut update_queue.node_update_queue);
            self.process_app_updates(&mut update_queue.app_update_queue);
            self.process_service_type_updates(&mut update_queue.service_type_update_queue);
            self.process_service_updates(&mut update_queue.service_update_queue);
            self.process_failover_unit_updates(&mut update_queue.failover_unit_update_queue);
            self.process_load_updates(&mut update_queue.load_update_queue);
        }

        // Let scheduler decide what phases will be run in this refresh
        // TODO: this should be run on each service domain, but for simplicity we can pack everything into one service domain
        let phases = self.scheduler.get_current_phases(now);

        let mut solutions = vec![];
        // For each phase generated by the scheduler,
        //  1. active PLB searcher to search for any actions
        //  2. activate solver to generate any solutions
        for phase in phases {
            self.searcher = Searcher::new(&self.cluster_snapshot);
            self.solver = Solver::new(&self.cluster_snapshot);
            let actions = self.searcher.generate_actions(phase);
            solutions = self.solver.generate_solutions(actions);
        }

        // TODO: give action generated by the solver back to FM (this will just be printing out the solution to the console for now)
        println!("Solutions generated: {:?}", solutions);

        Ok(solutions)
    }

    fn process_node_updates(&mut self, node_updates: &mut VecDeque<Node>) {
        while !node_updates.is_empty() {
            let node_update = node_updates.pop_front().unwrap();
            let node_id = node_update.node_id();
            self.cluster_snapshot
                .borrow_mut()
                .nodes
                .insert(node_id, node_update);
        }
    }

    fn process_app_updates(&mut self, app_updates: &mut VecDeque<Application>) {
        while !app_updates.is_empty() {
            let app_update = app_updates.pop_front().unwrap();
            let app_name = app_update.app_name();
            self.cluster_snapshot
                .borrow_mut()
                .apps
                .insert(String::from(app_name), app_update);
        }
    }

    fn process_service_type_updates(&mut self, service_type_updates: &mut VecDeque<ServiceType>) {
        while !service_type_updates.is_empty() {
            let service_type_update = service_type_updates.pop_front().unwrap();
            let service_type_name = service_type_update.service_type_name();
            self.cluster_snapshot
                .borrow_mut()
                .service_types
                .insert(String::from(service_type_name), service_type_update);
        }
    }

    fn process_service_updates(&mut self, service_updates: &mut VecDeque<Service>) {
        while !service_updates.is_empty() {
            let service_update = service_updates.pop_front().unwrap();
            let service_name = service_update.servcie_name();
            self.cluster_snapshot
                .borrow_mut()
                .services
                .insert(String::from(service_name), service_update);
        }
    }

    fn process_failover_unit_updates(
        &mut self,
        failover_unit_updates: &mut VecDeque<FailoverUnit>,
    ) {
        while !failover_unit_updates.is_empty() {
            let failover_unit_update = failover_unit_updates.pop_front().unwrap();
            let fu_id = failover_unit_update.id();
            self.cluster_snapshot
                .borrow_mut()
                .failover_units
                .insert(fu_id, failover_unit_update);
        }
    }

    fn process_load_updates(&mut self, load_updates: &mut VecDeque<LoadOrMoveCost>) {
        while !load_updates.is_empty() {
            let load_update = load_updates.pop_front().unwrap();
            let fu_id = load_update.id();
            self.cluster_snapshot
                .borrow_mut()
                .loads
                .insert(fu_id, load_update);
        }
    }

    /// Given a failover unit and 2 candicate secondary replicas, return the comparision result for promoting to primary
    /// A negative return value means Node 1 is preferred; a positive return value means Node 2 is preferred; 0 return value means
    /// 2 candidate nodes are equally preferred.
    ///
    /// The default CNFP algorithm is Dummy PLB
    /// TODO: add more CNFP algorithms and make an interface for user to choose
    pub fn compare_node_for_promotion(
        &self,
        _service_name: &str,
        _fu_id: Uuid,
        node1: NodeId,
        node2: NodeId,
    ) -> i32 {
        match node1.cmp(node2) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::vec_deque;
    use std::collections::HashMap;

    use crate::scheduler::Phase;
    use crate::scheduler::MIN_PLACEMENT_INTERVAL;

    use self::failoverunit::failover_unit::Replica;

    use super::*;

    fn create_empty_plb() -> PlacementAndLoadBalancing {
        PlacementAndLoadBalancing::new(vec![], vec![], vec![], vec![], vec![], vec![])
    }

    fn create_node_desc(node_id: u128) -> NodeDescription {
        let mut node_desc = NodeDescription::default();
        node_desc.node_instance.id = NodeId { id_value: node_id };
        node_desc
    }

    fn create_service_type_desc(service_type_name: &str) -> ServiceTypeDescription {
        let mut service_type_desc = ServiceTypeDescription::default();
        service_type_desc.name = String::from(service_type_name);
        service_type_desc
    }

    fn create_service_desc(service_type_name: &str, service_name: &str) -> ServiceDescription {
        let mut service_desc = ServiceDescription::default();
        service_desc.service_type_name = String::from(service_type_name);
        service_desc.service_name = String::from(service_name);
        service_desc
    }

    fn create_fu_desc(
        fu_id: Uuid,
        service_name: &str,
        replicas: HashMap<Uuid, Replica>,
        replica_diff: i32,
    ) -> FailoverUnitDescription {
        let mut fu_desc = FailoverUnitDescription::default();
        fu_desc.id = fu_id;
        fu_desc.service_name = String::from(service_name);
        fu_desc.replicas = replicas;
        fu_desc.replica_diff = replica_diff;
        fu_desc
    }

    #[test]
    fn test_dummy_placement_basic() {
        // Get a bunch of failover units with replica diff = 1
        // put them into PLB and expect the PLB to place them on the node with greatest node id
        let mut plb = create_empty_plb();

        plb.update_node(create_node_desc(0));
        plb.update_node(create_node_desc(1));
        plb.update_node(create_node_desc(2));

        plb.update_service_type(create_service_type_desc("Worker.ISO"));
        plb.update_service(create_service_desc("Worker.ISO", "LogicalServier"));
        plb.update_failover_unit(create_fu_desc(
            Uuid::from_u128(1),
            "LogicalServer",
            HashMap::new(),
            1,
        ));

        let initial_time = OffsetDateTime::now_utc();
        plb.scheduler
            .set_last_phase_time(initial_time, Phase::Placement);

        let solutions = plb.refresh(initial_time + MIN_PLACEMENT_INTERVAL).unwrap();

        assert_eq!(1, solutions.len());
        assert_eq!(
            Solution::AddReplica(format!(
                "Partition {:}: AddReplica on Node {:?}",
                Uuid::from_u128(1),
                NodeId::new(2)
            )),
            solutions[0]
        );
    }
}
