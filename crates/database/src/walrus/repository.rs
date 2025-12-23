use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::UpdateResult,
};

use crate::Database;

//one project has 1 blob (zip file) and 1 site
#[allow(clippy::module_name_repetitions)]
pub type DynProjectRepository = Arc<dyn ProjectRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait ProjectRepositoryTrait {
    async fn update_walrus_blob(
        &self,
        project_id: &str,
        blob_id: &str,
        identifier: &str,
        file_name: &str,
    ) -> Result<UpdateResult>;
    async fn update_walrus_site(&self, project_id: &str, site_id: &str) -> Result<UpdateResult>;
}
#[async_trait]
impl ProjectRepositoryTrait for Database {
    async fn update_walrus_blob(
        &self,
        project_id: &str,
        quilt_id: &str,
        identifier: &str,
        file_name: &str,
    ) -> Result<UpdateResult> {
        let id = ObjectId::parse_str(project_id)?;
        let filter = doc! {"_id": id};
        let update = doc! {
            "$set": {
                "quilt": {
                    "id": quilt_id,
                    "identifier": identifier,
                    "file_name": file_name,
                }
            }
        };
        let updated_doc = self.project_col.update_one(filter, update).await?;

        Ok(updated_doc)
    }
    async fn update_walrus_site(
        &self,
        project_id: &str,
        site_id: &str,
    ) -> Result<UpdateResult> {
        let id = ObjectId::parse_str(project_id)?;
        let filter = doc! {"_id": id};
        let new_doc = doc! {
            "$set":
                {
                    "site": {
                        "id": site_id,
                    }
                },
        };

        let updated_doc = self.project_col.update_one(filter, new_doc).await?;

        Ok(updated_doc)
    }
}
