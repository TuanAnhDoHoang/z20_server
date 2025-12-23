use std::{sync::Arc};

use anyhow::Result;
use axum::async_trait;
use database::walrus::repository::DynProjectRepository;
use mongodb::results::UpdateResult;

use crate::dtos::walrus_dto::BlobUpdateDto;

#[allow(clippy::module_name_repetitions)]
pub type DynProjectService = Arc<dyn ProjectServiceTrait + Send + Sync>;

#[async_trait]
#[allow(clippy::module_name_repetitions)]
pub trait ProjectServiceTrait {
    // async fn get_current_user(&self, user_id: &str) -> AppResult<Option<User>>;

    async fn update_walrus_blob(&self, blob_update_dto: &BlobUpdateDto)
        -> Result<UpdateResult>;

    async fn update_walrus_site(&self, blob_update_dto: &BlobUpdateDto) -> Result<UpdateResult>;
}

#[derive(Clone)]
pub struct ProjectService {
    repository: DynProjectRepository,
}

impl ProjectService {
    pub fn new(repository: DynProjectRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ProjectServiceTrait for ProjectService {
    async fn update_walrus_blob(
        &self,
        blob_update_dto: &BlobUpdateDto,
    ) -> Result<UpdateResult> {
        let project_id = blob_update_dto.project_id();

        let identifier = blob_update_dto.identifier();
        let blob_id: &String = blob_update_dto.blob_id();
        let file_name = blob_update_dto.file_name();
        
        self.repository.update_walrus_blob(project_id, blob_id, identifier, file_name).await
    }

    async fn update_walrus_site(&self, blob_update_dto: &BlobUpdateDto) -> Result<UpdateResult> {
        let project_id = blob_update_dto.project_id();
        let site_id = blob_update_dto.blob_id(); // Giả sử site_id lấy từ đây

        self.repository.update_walrus_site(project_id, site_id).await
    }
}
