use env_set_up::model::Employee;
use eventsourcing_module::employee_command::model::EmployeeCommand;

#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[event_type_version("1.0")]
#[event_source("")]
pub enum EmployeeEvent {
    EmployeeCreated(Employee),
    EmployeeUpdated { salary: u64 },
    EmployeeDeleted,
}

impl From<EmployeeCommand> for EmployeeEvent {
    fn from(source: EmployeeCommand) -> Self {
        match source {
            EmployeeCommand::CreateEmployee(Employee) =>
                EmployeeEvent::EmployeeCreated(Employee),
            EmployeeCommand::UpdateEmployee { emp_id, emp_salary } =>
                EmployeeEvent::EmployeeUpdated { salary },
            EmployeeCommand::DeleteEmployee {emp_id} =>
                EmployeeEvent::EmployeeDeleted,
            _ => "invalid command",
        }

    }
}