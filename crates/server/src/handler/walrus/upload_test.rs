#[cfg(test)]
mod test {
    use crate::{dtos::file_dto::BlobInfomation, handler::walrus::{blob_handler::blob_upload, site_handler::upload_walrus_site}};
    #[tokio::test]
    async fn upload_blob_to_walrus() {
        let project_name = "z20";
        let blob_name = "index.html";
        let client_address = "0xcfd0a4769d3d4a896812c8a233a10dee1258b998f284c76a782838fea13274a0";
        let blob_father_path = "/home/anhdoo/codespace/walrus/docker_walrus/test-site";

        let mut blob_info = BlobInfomation::default();
        blob_info.set_project_name(project_name.into());
        blob_info.set_client_address(client_address.into());
        blob_info.set_blob_name(blob_name.into());
        blob_info.set_blob_father_path(blob_father_path.into());

        let blob_upload_response = blob_upload(&blob_info).await;
        println!("Upload successfully: {blob_upload_response:?}");
    }
    #[tokio::test]
    async fn upload_blob_to_walrus_site() {
        let blob_father_path = "/home/anhdoo/codespace/walrus/docker_walrus/walrus-snake";
        match upload_walrus_site(blob_father_path).await {
            Ok(result) => {
                if result.success {
                    println!(
                        "Site published successfully! Object ID: {:?}",
                        result.object_id
                    );
                    // Có thể trả về JSON cho client với site info
                    // return Json(result);
                } else {
                    eprintln!("Publish failed: {}", result.message);
                    // Có thể vẫn trả 200 nếu muốn, hoặc 500
                }
            }
            Err(err) => {
                eprintln!("Error publishing to Walrus: {err}");
            }
        }
    }
}
