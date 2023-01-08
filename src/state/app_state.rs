use crate::core::database_core;

#[derive(Debug)]
pub struct AppState {
    pub database: database_core::DatabaseCore,
}

impl AppState {
    pub fn new(url: String) -> AppState {
        AppState {
            database: database_core::DatabaseCore::new(url),
        }
    }
}
