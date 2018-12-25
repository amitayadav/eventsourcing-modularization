use env_set_up::model::Employee;

#[derive(Debug)]
pub enum EmployeeCommand {
    CreateEmployee(Employee),
    UpdateEmployee{emp_id: u64,emp_salary: u64},
    DeleteEmployee{emp_id: u64},
    GetEmployee{emp_id: u64}
}