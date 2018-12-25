#![feature(attr_literals)]
#![feature(custom_attribute)]
#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
extern crate chrono;
#[macro_use]
extern crate eventsourcing;
#[macro_use]
extern crate eventsourcing_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate uuid;
extern crate indexmap;

pub mod env_set_up {
    pub mod connection;
    pub mod keyspace;
    pub mod table;
    pub mod model;
}

pub mod eventsourcing_module {
    pub mod employee_command {
        pub mod model;
    }

    pub mod employee_event {
        pub mod model;
    }

    pub mod employee_entity {
        pub mod behaviour;
    }

    pub mod employee_repository {
        pub mod create;
        pub mod remove;
        pub mod update;
        pub mod display;
    }

    pub mod employee_state {
        pub mod model;
    }
}