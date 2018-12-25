use eventsourcing::{eventstore::MemoryEventStore, prelude::*, Result};
use eventsourcing_module::employee_event::model::*;
use eventsourcing_module::employee_command::model::*;
use eventsourcing_module::employee_state::model::*;
use env_set_up::model::*;

pub struct Employees;

impl Aggregate for Employees {
    type Event = EmployeeEvent;
    type Command = EmployeeCommand;
    type State = EmployeeState;

    fn apply_event(state: &Self::State, evt: Self::Event) -> Result<Self::State> {
        let emp_data = match evt {
            EmployeeEvent::EmployeeCreated(Employee) => EmployeeState {
                emp: Employee,
                generation: state.generation + 1,
            },
            EmployeeEvent::EmployeeUpdated(salary) => EmployeeState {
                emp: Employee {
                    emp_salary: salary,
                    ..Employee
                },
                generation: state.generation + 1,
            },
            EmployeeEvent::EmployeeDeleted(emp_id) => EmployeeState {
                emp: Employee {
                    emp_id: 0,
                    emp_name: "",
                    emp_salary: 0,
                },
                generation: state.generation + 1,
            },
            _ => "invalid event",
        };
        Ok(emp_data)
    }

    fn handle_command(_state: &Self::State, cmd: Self::Command) -> Result<Vec<Self::Event>> {
        let evts = match cmd {
            BankCommand::DepositFunds(acct, amt) => vec![BankEvent::FundsDeposited(acct, amt)],
            BankCommand::WithdrawFunds(acct, amt) => vec![BankEvent::FundsWithdrawn(acct, amt)],
        };
        Ok(evts)
    }
}