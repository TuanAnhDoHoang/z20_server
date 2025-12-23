pub mod walrus;

use std::sync::Arc;

use mongodb::{Client, Collection};
use tracing::info;

use utils::{AppConfig, AppResult};

use crate::walrus::model::Project;

#[derive(Clone, Debug)]
pub struct Database {
    pub project_col: Collection<Project>,
}

impl Database {
    /// Creates a new `Database` instance.
    ///
    /// # Arguments
    ///
    /// * `config` - An `Arc` containing the application configuration.
    ///
    /// # Returns
    ///
    /// * `AppResult<Self>` - A result containing the `Database` instance or an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the `MongoDB` client cannot be initialized
    /// or if the specified database or collection cannot be accessed.
    pub async fn new(config: Arc<AppConfig>) -> AppResult<Self> {
        let client = Client::with_uri_str(&config.mongo_uri).await?;
        let db = client.database(&config.mongo_db);
        // let project_col: Collection<User> = db.collection("User");
        let project_col = db.collection("projects");

        println!("connect to {}::{}", &config.mongo_uri, &config.mongo_db);

        info!("initializing database connection...");

        Ok(Database { project_col })
    }
}
