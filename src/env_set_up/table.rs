use env_set_up::connection::*;
use cdrs::query::QueryExecutor;

pub fn create_table(session: &CurrentSession) {
    let create_table_cql =
        "CREATE TABLE IF NOT EXISTS employee_ks.employee_event (emp_id int PRIMARY KEY , \
     name text, salary decimal);";
    session
        .query(create_table_cql)
        .expect("Table creation error");
}