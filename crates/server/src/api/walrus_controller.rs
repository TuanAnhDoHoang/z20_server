use std::path::Path;

use axum::{routing::post, Json, Router};
use axum_extra::extract::Multipart;
use utils::AppResult;

use crate::{
    dtos::walrus_dto::BlobUploadResponse,
    handler::walrus::{
        blob_handler::blob_upload,
        file_handler::handle_upload_chunk,
        site_handler::{upload_walrus_site, WalrusPublishResult},
    },
};

pub struct WalrusController;
impl WalrusController {
    pub fn app() -> Router {
        Router::new()
            .route("/blobs", post(Self::upload_blobs))
            .route("/site", post(Self::upload_site))
    }
    pub async fn upload_blobs(multipart: Multipart) -> AppResult<Json<BlobUploadResponse>> {
        let blob_info = handle_upload_chunk(multipart)
            .await
            .map_err(|_| utils::AppError::InternalServerError)?;

        let path = Path::new(blob_info.blob_father_path()).join(blob_info.blob_name());

        if !path.exists() {
            return Err(utils::AppError::InternalServerError);
        }

        //validate
        let upload_result = blob_upload(&blob_info).await?;

        Ok(Json(upload_result))
    }

    pub async fn upload_site(multipart: Multipart) -> AppResult<Json<WalrusPublishResult>> {
        let blob_info = handle_upload_chunk(multipart)
            .await
            .map_err(|_| utils::AppError::InternalServerError)?;

        let path = Path::new(blob_info.blob_father_path());

        if !path.exists() {
            return Err(utils::AppError::InternalServerError);
        }

        let response = upload_walrus_site(blob_info.blob_father_path()).await?;
        Ok(Json(response))
    }
}
