use std::path::Path;

use anyhow::{anyhow, Result};
use reqwest::{multipart, Client};
use tokio::{fs::File, io::AsyncReadExt};

use crate::dtos::{file_dto::BlobInfomation, walrus_dto::{BlobUploadResponse, QuiltUploadResponse}};

pub async fn blob_upload(
    blob_info: &BlobInfomation
) -> Result<BlobUploadResponse> {
    let client_address = blob_info.client_address();
    let project_name = blob_info.project_name();
    let blob_name = blob_info.blob_name();
    let blob_father_path = blob_info.blob_father_path();


    let path_blob = Path::new(blob_father_path).join(blob_name);
    if !path_blob.exists() {
        // Trả về lỗi nếu không tìm thấy thư mục (Bạn có thể dùng NotFound error nếu có)
        eprintln!("Error blob upload: file not found");
        return Err(anyhow!(""));
    }
    let publisher = "https://publisher.walrus-testnet.walrus.space";
    let endpoint = format!("{publisher}/v1/quilts");

    let mut file = File::open(path_blob).await?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).await?;

    let form = multipart::Form::new().part(
        project_name.to_owned(),
        multipart::Part::bytes(contents)
            .file_name(blob_name.to_string()) // Tên file hiển thị (tùy chọn)
            .mime_str("application/octet-stream")?, // Hoặc mime phù hợp
    );

    let epochs = 5;

    let client = Client::new();
    let response = client
        .put(format!("{endpoint}?epochs={epochs}&send_object_to={client_address}",))
        .multipart(form)
        .send()
        .await?;

    if response.status().is_success() {
        let upload_response: QuiltUploadResponse = response.json().await?;
        let mut result = BlobUploadResponse::default();
        result.set_quilt_upload_response(upload_response);
        Ok(result)
    } else {
        eprintln!("Error: {}", response.status());
        eprintln!("Body: {:?}", response.text().await?);
        Err(anyhow!(""))
    }
}
