use std::fs::{self, File, OpenOptions};

use anyhow::{anyhow, Result};
use axum_extra::extract::Multipart;
use std::io::Write;

use crate::dtos::file_dto::BlobInfomation;

pub async fn handle_upload_chunk(mut multipart: Multipart) -> Result<(bool, BlobInfomation)> {
    let mut upload_id = String::new();
    let mut identifier = String::new();
    let mut file_name = String::new();
    let mut chunk_number = 0;
    let mut total_chunks = 0;
    let mut chunk_data = Vec::new();

    // Process multipart form data
    while let Some(field) = match multipart.next_field().await {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Error reading multipart field: {err:?}");
            return Err(anyhow!(""));
        }
    } {
        let field_name = field.name().unwrap_or_default().to_string();
        match field_name.as_str() {
            "uploadId" => {
                upload_id = field.text().await.unwrap_or_default();
                if upload_id.is_empty() {
                    return Err(anyhow!("Upload Id not found"));
                }
            }
            "identifier" => {
                identifier = field.text().await.unwrap_or_default();
                if identifier.is_empty() {
                    return Err(anyhow!("Identifier not found"));
                }
            }
            "fileName" => {
                file_name = field.text().await.unwrap_or_default();
                file_name = sanitize_filename(&file_name);
            }
            "chunkNumber" => {
                chunk_number = field.text().await.unwrap_or_default().parse().unwrap_or(0);
            }
            "totalChunks" => {
                total_chunks = field.text().await.unwrap_or_default().parse().unwrap_or(0);
            }
            "chunk" => {
                match field.bytes().await {
                    Ok(bytes) => chunk_data = bytes.to_vec(), // Convert Bytes to Vec<u8>
                    Err(err) => {
                        eprintln!("Error reading chunk data: {err:?}");
                        return Err(anyhow!(""));
                    }
                }
            }
            _ => {}
        }
    }

    // Validate that required fields are provided
    if upload_id.is_empty() || file_name.is_empty() || chunk_data.is_empty() || total_chunks == 0 {
        eprintln!("Missing required fields: file_name: {file_name}, chunk_data: {chunk_data:?}");
        return Err(anyhow!(""));
    }

    // Create a temporary directory to store the file chunks
    let temp_dir = format!("/tmp/upload/temp/{upload_id}");
    create_dir(&temp_dir)?;

    // Save the chunk to a temporary file
    let chunk_path = format!("{temp_dir}/chunk_{chunk_number:05}");
    let mut file = create_file(&chunk_path)?;

    if let Err(err) = file.write_all(&chunk_data) {
        eprintln!("Failed to write chunk data: {chunk_path:?}, Error: {err:?}");
        return Err(anyhow!(""));
    }

    // If all chunks are uploaded, assemble the file
    if is_upload_complete(&temp_dir, total_chunks) {
        let project_dir = format!("/tmp/uploads/{upload_id}");
        create_dir(&project_dir)?;

        if let Err(err) = assemble_file(&temp_dir, &project_dir, &file_name, total_chunks) {
            eprintln!("Failed to assemble file: {file_name:?}, Error: {err:?}");
            return Err(anyhow!(""));
        }

        let mut result = BlobInfomation::default();
        result.set_identifier(identifier);
        result.set_blob_name(file_name);
        result.set_blob_father_path(project_dir);
        Ok((true, result))

    } else {
        Ok((false, BlobInfomation::default()))
    }
}

// Sanitize filename to avoid directory traversal attacks
pub fn sanitize_filename(filename: &str) -> String {
    filename.replace(&['/', '\\'][..], "").replace("..", "")
}

#[allow(clippy::suspicious_open_options)]
fn assemble_file(
    temp_dir: &str,
    project_dir: &str,
    file_name: &str,
    total_chunks: usize,
) -> std::io::Result<()> {
    let output_path = format!("{project_dir}/{file_name}");
    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&output_path)?;

    for chunk_number in 0..total_chunks {
        let chunk_path = format!("{temp_dir}/chunk_{chunk_number:05}");
        let chunk_data = fs::read(&chunk_path)?;
        output_file.write_all(&chunk_data)?;
    }

    // Clean up the temporary chunks
    fs::remove_dir_all(temp_dir)?;

    Ok(())
}

//dir is full dir: /tmp/anhdoo.png
fn create_dir(full_dir: &str) -> Result<()> {
    if let Err(err) = fs::create_dir_all(full_dir) {
        eprintln!("Failed to create temp directory: {full_dir:?}, Error: {err:?}");
        return Err(anyhow!(""));
    }
    Ok(())
}

fn create_file(full_dir: &str) -> Result<File> {
    match File::create(&full_dir) {
        Ok(f) => Ok(f),
        Err(err) => {
            eprintln!("Failed to create file: {:?}, Error: {:?}", full_dir, err);
            return Err(anyhow!(""));
        }
    }
}

fn is_upload_complete(temp_dir: &str, total_chunks: usize) -> bool {
    match fs::read_dir(temp_dir) {
        Ok(entries) => entries.count() == total_chunks,
        Err(_) => false,
    }
}
