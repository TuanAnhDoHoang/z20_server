use std::{collections::HashMap, path::Path};

use axum::{extract::Query, routing::post, Extension, Json, Router};
use axum_extra::extract::Multipart;
use utils::{AppError, AppResult};

use crate::{
    dtos::{file_dto::RequestUpload, walrus_dto::{BlobUpdateDto, BlobUploadResponse}},
    handler::walrus::{
        blob_handler::blob_upload,
        file_handler::handle_upload_chunk,
        site_handler::{WalrusPublishResult, upload_walrus_site},
    },
    services::Services,
};

pub struct WalrusController;
impl WalrusController {
    pub fn app() -> Router {
        Router::new()
            // .route("/blobs", post(Self::upload_blobs))
            .route("/site", post(Self::upload_site))
    }
    // pub async fn upload_blobs(
    //     Extension(services): Extension<Services>,
    //     Query(query): Query<HashMap<String, String>>,
    //     multipart: Multipart,
    // ) -> AppResult<Json<BlobUploadResponse>> {
    //     let client_address = query
    //         .get("clientAddress")
    //         .ok_or_else(|| AppError::BadRequest("Missing clientAddress".to_string()));
    //     let project_id = query
    //         .get("projectId")
    //         .ok_or_else(|| AppError::BadRequest("Missing projectId".to_string()));
    //     //validation

    //     let (status, mut blob_info) = handle_upload_chunk(multipart)
    //         .await
    //         .map_err(|_| utils::AppError::InternalServerError)?;

    //     if !status {
    //         let mut response = BlobUploadResponse::default();
    //         response.set_status(false);
    //         return Ok(Json(response));
    //     }

    //     blob_info.set_client_address(client_address.unwrap().to_owned());

    //     let path = Path::new(blob_info.blob_father_path()).join(blob_info.blob_name());

    //     if !path.exists() {
    //         return Err(utils::AppError::InternalServerError);
    //     }

    //     //validate
    //     let upload_result = blob_upload(&blob_info).await?;

    //     let stored_quilt_blobs = upload_result.quilt_upload_response().stored_quilt_blobs();
    //     if stored_quilt_blobs.is_empty() {
    //         return Err(AppError::BadRequest("Your blob upload fail".to_string()));
    //     }
    //     let first_quilt = stored_quilt_blobs.first().unwrap();

    //     let mut blob_update_dto = BlobUpdateDto::default();
    //     blob_update_dto.set_project_id(project_id.unwrap().to_owned());
    //     blob_update_dto.set_blob_id(first_quilt.quilt_patch_id().to_owned());
    //     blob_update_dto.set_identifier(blob_info.identifier().to_owned());
    //     blob_update_dto.set_file_name(blob_info.blob_name().to_owned());

    //     let project_service = services.project;
    //     match project_service.update_walrus_blob(&blob_update_dto).await {
    //         Ok(_) => Ok(Json(upload_result)),
    //         Err(_) => Err(AppError::InternalServerError),
    //     }
    // }

    pub async fn upload_site(
        Extension(services): Extension<Services>,
        Query(query): Query<HashMap<String, String>>,
        Json(req) : Json<RequestUpload>
    ) -> AppResult<Json<WalrusPublishResult>> {
        let client_address = query
            .get("clientAddress")
            .ok_or_else(|| AppError::BadRequest("Missing clientAddress".to_string()));
        let project_id = query
            .get("projectId")
            .ok_or_else(|| AppError::BadRequest("Missing projectId".to_string()));

        println!("Request content: {req:?}");

        let (status, mut blob_info) = handle_upload_chunk(req)
            .await
            .map_err(|_| utils::AppError::InternalServerError)?;

        if !status {
            let mut response = WalrusPublishResult::default();
            response.success = false;
            return Ok(Json(response));
        }

        blob_info.set_client_address(client_address.unwrap().to_owned());

        let path = Path::new(blob_info.blob_father_path());

        if !path.exists() {
            return Err(utils::AppError::InternalServerError);
        }

        println!("blob_info: {:?}", blob_info);

        let response = upload_walrus_site(blob_info.blob_father_path()).await?;
        
        println!("Upload response: {:?}", response);

        // if response.success {
        //     let mut blob_update_dto = BlobUpdateDto::default();
        //     blob_update_dto.set_project_id(project_id.unwrap().to_owned());
        //     blob_update_dto.set_blob_id(response.object_id.clone().unwrap());

        //     println!("Update to database...");

        //     let project_service = services.project;
        //     match project_service.update_walrus_site(&blob_update_dto).await {
        //         Ok(_) => Ok(Json(response)),
        //         Err(e) => Err(AppError::InternalServerErrorWithContext(e.to_string())),
        //     }
        // } else {
        //     Err(AppError::BadRequest("Your site upload fail".to_string()))
        // }
        Ok(Json(response))
    }
}
