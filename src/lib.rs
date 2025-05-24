pub mod controller {
    pub mod user_controller;
    pub mod profile_controller;
}

pub mod factory;

pub mod model {
    pub mod user;
    pub mod profile;
}

pub mod repository {
    pub mod user_repository;
    pub mod profile_repository;
}

pub mod routes;

pub mod service {
    pub mod user_service;
    pub mod profile_service;
}

pub mod schema;
