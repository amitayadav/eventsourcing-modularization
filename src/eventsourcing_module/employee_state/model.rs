use env_set_up::model::*;
use eventsourcing::{eventstore::MemoryEventStore, prelude::*, Result};

#[derive(Debug)]
pub struct EmployeeState {
    pub emp: Employee,
    pub generation: u64
}

impl AggregateState for EmployeeState {
    fn generation(&self) -> u64 {
        self.generation
    }
}