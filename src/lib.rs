pub mod config;
pub mod error;
pub mod db;
pub mod models;
pub mod schema;
pub mod handlers;
pub mod routes;
pub mod services;
pub mod utils;

#[cfg(test)]

mod tests {
    pub mod chat_service_tests;
    pub mod metrics_tests;
}

mod tests;

