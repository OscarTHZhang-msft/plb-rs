//! This module contains the implementation of the PLBScheduler and its scheduling phases

use time::{Duration, OffsetDateTime};

/// Minimum duration between 2 placement phases
pub const MIN_PLACEMENT_INTERVAL: Duration = Duration::new(3, 0);
/// Minimum duration between 2 load balancing phases
const MIN_BALANCING_INTERVAL: Duration = Duration::new(10, 0);
/// Minimu duration between 2 constraint check phases
const MIN_CONSTRAINT_CHECK_INTERVAL: Duration = Duration::new(5, 0);

/// Phase represents the PLB scheduling phases. There are 3 top-level phases for PLBScheduler to schedule:
///     1. Placement
///     2. LoadBalancing
///     3. ConstraintCheck
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Placement,
    LoadBalancing,
    ConstraintCheck,
}

/// PLBScheduler initiates the scheduling phase and action for the PLB
#[derive(Debug, Clone)]
pub struct PLBScheduler {
    current_phase: Option<Phase>,
    last_placement_time: OffsetDateTime,
    last_balancing_time: OffsetDateTime,
    last_constraint_time: OffsetDateTime,
}

impl Default for PLBScheduler {
    /// Initialize the PLBScheduler instance to the default state
    fn default() -> Self {
        PLBScheduler {
            current_phase: None,
            last_placement_time: OffsetDateTime::UNIX_EPOCH,
            last_balancing_time: OffsetDateTime::UNIX_EPOCH,
            last_constraint_time: OffsetDateTime::UNIX_EPOCH,
        }
    }
}

impl PLBScheduler {
    /// Initialize the PLBScheduler instance by setting the last action time for all 3 phases
    /// to the current timestamp passed in by caller
    pub fn new(now: OffsetDateTime) -> Self {
        PLBScheduler {
            current_phase: None,
            last_placement_time: now,
            last_balancing_time: now,
            last_constraint_time: now,
        }
    }

    /// Cleanup the timer to the timestamp of now so that the counting will restart from now
    pub fn reset(&mut self, now: OffsetDateTime) {
        self.current_phase = None;
        self.last_placement_time = now;
        self.last_balancing_time = now;
        self.last_constraint_time = now;
    }

    /// Get a list of current phases that is due for the PLB by comparing the current timestamp with the last timestamps for
    /// all 3 PLB phases
    pub fn get_current_phases(&mut self, now: OffsetDateTime) -> Vec<Phase> {
        let mut phases = vec![];
        if now - self.last_placement_time >= MIN_PLACEMENT_INTERVAL {
            phases.push(Phase::Placement);
            self.last_placement_time = now;
        }
        if now - self.last_balancing_time >= MIN_BALANCING_INTERVAL {
            phases.push(Phase::LoadBalancing);
            self.last_balancing_time = now;
        }
        if now - self.last_constraint_time >= MIN_CONSTRAINT_CHECK_INTERVAL {
            phases.push(Phase::ConstraintCheck);
            self.last_constraint_time = now;
        }

        phases
    }

    pub(super) fn set_last_phase_time(&mut self, now: OffsetDateTime, phase: Phase) {
        match phase {
            Phase::Placement => self.last_placement_time = now,
            Phase::LoadBalancing => self.last_balancing_time = now,
            Phase::ConstraintCheck => self.last_constraint_time = now,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scheduler;

    use super::*;

    #[test]
    fn test_scheduling_phase() {
        let initial_timestamp = OffsetDateTime::now_utc();
        let mut scheduler = PLBScheduler::new(initial_timestamp);
        // Initially there should be no phase scheduled
        assert!(scheduler.get_current_phases(initial_timestamp).is_empty());

        let time1 = initial_timestamp + Duration::new(1, 0);
        // No phase due
        assert!(scheduler.get_current_phases(time1).is_empty());

        let time2 = initial_timestamp + MIN_PLACEMENT_INTERVAL;
        assert_eq!(vec![Phase::Placement], scheduler.get_current_phases(time2));
    }
}
