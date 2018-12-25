use env_set_up::connection::*;
use cdrs::query::QueryExecutor;

pub fn create_keyspace(session: &CurrentSession) {
    let create_ks: &'static str = "CREATE KEYSPACE IF NOT EXISTS employee.ks WITH REPLICATION = { \
                                 'class' : 'SimpleStrategy', 'replication_factor' : 1 };";
    session.query(create_ks);
}