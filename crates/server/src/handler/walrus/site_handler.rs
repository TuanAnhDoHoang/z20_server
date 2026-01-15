use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Stdio;
use tokio::io::AsyncReadExt;
use tokio::process::Command as TokioCommand;

#[derive(Debug, serde::Serialize, Default)]
pub struct WalrusPublishResult {
    pub success: bool,
    pub site_name: Option<String>,
    pub object_id: Option<String>,
    pub message: String,
    pub raw_json: Option<Value>, // Toàn bộ nội dung ws-resources.json nếu cần
}

pub async fn upload_walrus_site(project_dir: &str) -> Result<WalrusPublishResult> {
    println!("=================Log_from_upload_walrus_site===========");
    let project_path = Path::new(project_dir);
    let ws_resources_path = project_path.join("ws-resources.json");

    // Xóa file cũ nếu tồn tại (tránh trường hợp cũ còn sót)
    let _ = fs::remove_file(&ws_resources_path);

    // Chạy Docker publish
    // let mut child = TokioCommand::new("docker")
    //     .arg("run")
    //     .arg("--rm")
    //     .arg("-v")
    //     .arg(format!("{project_dir}:/site"))
    //     // .arg("-v")
    //     // .arg(format!("{home}:/root/.sui/sui_config"))
    //     .arg("walrus-site-builder:testnet")
    //     .arg("publish")
    //     .arg("/site")
    //     .arg("--epochs")
    //     .arg("1")
    //     .stdout(Stdio::piped())
    //     .stderr(Stdio::piped())
    //     .spawn()?;

    println!("project path: {project_path:?}");

    // Chạy Commandline publish
    let site_config_path = "/etc/walrus/sites-config.yaml";
    let mut child = TokioCommand::new("site-builder")
        .arg("--config")
        .arg(site_config_path)
        .arg("publish")
        .arg(project_dir)
        .arg("--epochs")
        .arg("1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Đọc stderr realtime để log lỗi nếu có
    if let Some(mut stderr) = child.stderr.take() {
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                match stderr.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        eprint!("Read stderr{}", String::from_utf8_lossy(&buf[..n]));
                    }
                    Err(_) => break,
                }
            }
        });
    }

    // Chờ Docker kết thúc
    let status = child.wait().await?;

    if !status.success() {
        return Ok(WalrusPublishResult {
            success: false,
            site_name: None,
            object_id: None,
            message: "Docker publish thất bại (xem log stderr)".to_string(),
            raw_json: None,
        });
    }

    // Kiểm tra file ws-resources.json có được tạo không
    if !ws_resources_path.exists() {
        return Ok(WalrusPublishResult {
            success: false,
            site_name: None,
            object_id: None,
            message: "Publish thành công nhưng không tìm thấy ws-resources.json".to_string(),
            raw_json: None,
        });
    }

    // Đọc và parse JSON
    let content = fs::read_to_string(&ws_resources_path)?;
    let json: Value = serde_json::from_str(&content)?;

    // Extract thông tin cần thiết (dựa trên cấu trúc thực tế của file)
    let site_name = json
        .get("site_name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let object_id = json
        .get("object_id")
        .or_else(|| json.get("site_object_id"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    println!("\njson result: {json:?}\n");


    Ok(WalrusPublishResult {
        success: true,
        site_name,
        object_id,
        message: "Publish thành công lên Walrus testnet!".to_string(),
        raw_json: Some(json),
    })
}
