pub(crate) mod walrus_service;

use database::Database;
use std::sync::Arc;
use tracing::info;

use crate::services::walrus_service::{DynProjectService, ProjectService};


#[derive(Clone)]
pub struct Services {
    pub project: DynProjectService,
}

impl Services {
    pub fn new(db: Database) -> Self {
        info!("initializing services...");
        let repository = Arc::new(db);

        let project = Arc::new(ProjectService::new(repository.clone())) as DynProjectService;

        Self { project }
    }
}
