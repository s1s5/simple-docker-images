use std::collections::HashMap;
use std::io::Write as _;
use std::net::SocketAddr;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::Result;
use axum::extract::Json;
use axum::response::IntoResponse;
use axum::{Router, http::StatusCode, routing::post};
use log::{error, info};
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::signal;

#[derive(Deserialize, Debug)]
struct Cmd {
    cmd: Vec<String>,
    stdin_url: Option<String>,

    stdout_url: Option<String>,
    stderr_url: Option<String>,
}

#[derive(Deserialize, Debug)]
struct CmdList {
    input_file_mapping: Option<HashMap<String, String>>,
    cmd_list: Vec<Cmd>,
    output_file_mapping: Option<HashMap<String, String>>,
}

async fn handler(Json(cmd_list): Json<CmdList>) -> impl IntoResponse {
    info!("cmd_list: {cmd_list:#?}");
    // 1. テンポラリディレクトリの作成
    let temp_dir = env::temp_dir().join(uuid::Uuid::new_v4().to_string());
    if let Err(err) = fs::create_dir_all(&temp_dir) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create temporary directory: {}", err),
        );
    }

    info!("Created temporary directory: {}", temp_dir.display());

    let r = execute_main(cmd_list, &temp_dir).await;

    // 最後にテンポラリディレクトリを削除
    if let Err(err) = fs::remove_dir_all(&temp_dir) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to remove temporary directory: {}", err),
        );
    }
    info!("Cleaned up temporary directory: {}", temp_dir.display());

    match r {
        Ok(()) => (StatusCode::OK, "OK".to_string()),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to execute main command: {}", err),
        ),
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    env_logger::init();

    let cors = tower_http::cors::CorsLayer::new()
        .allow_credentials(false)
        .allow_headers(tower_http::cors::Any)
        .allow_origin(tower_http::cors::AllowOrigin::mirror_request());
    let router = Router::new().route("/", post(handler)).layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("server listing {addr:?}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    info!("terminating...");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    let sig_int = async {
        signal::unix::signal(signal::unix::SignalKind::interrupt())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {log::debug!("'Ctrl C' received")},
        _ = sig_int => {log::debug!("SIGINT received")},
        _ = terminate => {log::debug!("SIGTERM received")},
    }
}

async fn download_file(url: &str, path: &Path, temp_dir: &Path) -> Result<()> {
    info!("Downloading {} to {}", url, path.display());
    let response = reqwest::get(url).await?;
    let mut file = File::create(temp_dir.join(path)).await?;
    file.write_all(&response.bytes().await?).await?;
    Ok(())
}

async fn upload_file(url: &str, path: &Path, temp_dir: &Path) -> Result<()> {
    info!("Uploading {} to {}", path.display(), url);
    let file = tokio::fs::read(temp_dir.join(path)).await?;
    reqwest::Client::new().put(url).body(file).send().await?;
    Ok(())
}

async fn upload_directory_as_zip(url: &str, dir_path: &Path, temp_dir: &Path) -> Result<()> {
    info!("Zipping and uploading {} to {}", dir_path.display(), url);
    let local_filename = format!("temp_{}.zip", uuid::Uuid::new_v4());
    let tmp_file = temp_dir.join(&local_filename);

    let file = fs::File::create(&tmp_file)?;
    let mut zip = zip::ZipWriter::new(file);
    let options: zip::write::FileOptions<'static, ()> =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let mut paths = Vec::new();
    let mut stack = vec![dir_path.to_path_buf()];

    while let Some(path) = stack.pop() {
        if path.is_file() {
            paths.push(path);
        } else if path.is_dir() {
            for entry in fs::read_dir(path)? {
                stack.push(entry?.path());
            }
        }
    }

    for path in &paths {
        let name = path.strip_prefix(dir_path)?;
        zip.start_file(name.to_string_lossy(), options)?;
        let mut f = fs::File::open(path)?;
        std::io::copy(&mut f, &mut zip)?;
    }
    zip.finish()?;

    upload_file(url, &PathBuf::from(local_filename), temp_dir).await?;

    Ok(())
}

async fn execute_cmd(cmd_config: Cmd, temp_dir: &Path) -> Result<()> {
    // 4. cmdを実行
    info!("Executing main command: {:?}", cmd_config.cmd);
    let mut main_cmd = Command::new(&cmd_config.cmd[0]);
    main_cmd.args(&cmd_config.cmd[1..]);
    main_cmd.current_dir(temp_dir);

    // stdinの設定
    let stdin_content: Vec<u8> = if let Some(stdin_url) = &cmd_config.stdin_url {
        info!("Downloading stdin from: {}", stdin_url);
        let response = reqwest::get(stdin_url).await?;
        response.bytes().await?.into()
    } else {
        vec![]
    };

    main_cmd.stdin(Stdio::piped());
    if cmd_config.stdout_url.is_some() {
        main_cmd.stdout(Stdio::piped());
    }
    if cmd_config.stderr_url.is_some() {
        main_cmd.stderr(Stdio::piped());
    }

    let mut child = main_cmd.spawn()?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(&stdin_content)?;
    }
    let output = child.wait_with_output()?;

    // stdoutとstderrをファイルに保存
    let stdout_path = temp_dir.join(format!("stdout_{}.log", uuid::Uuid::new_v4()));
    let stderr_path = temp_dir.join(format!("stderr_{}.log", uuid::Uuid::new_v4()));
    tokio::fs::write(&stdout_path, &output.stdout).await?;
    tokio::fs::write(&stderr_path, &output.stderr).await?;

    // stdout_urlへの出力
    if let Some(stdout_url) = &cmd_config.stdout_url {
        upload_file(stdout_url, &stdout_path, temp_dir).await?;
    } else if !output.stdout.is_empty() {
        info!("Stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    }

    // stderr_urlへの出力
    if let Some(stderr_url) = &cmd_config.stderr_url {
        upload_file(stderr_url, &stderr_path, temp_dir).await?;
    } else if !output.stderr.is_empty() {
        error!("Stderr:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    if !output.status.success() {
        Err(anyhow::anyhow!(
            "Main command failed with exit code {:?}. cmd={:?}",
            output.status.code(),
            cmd_config
        ))?;
    }

    Ok(())
}

async fn execute_main(cmd_config: CmdList, temp_dir: &Path) -> Result<()> {
    // 3. input_file_mappingからファイルをダウンロード
    for (path, url) in cmd_config.input_file_mapping.unwrap_or_default() {
        let target_path = temp_dir.join(path);
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }
        download_file(&url, &target_path, temp_dir).await?;
    }

    for cmd in cmd_config.cmd_list {
        execute_cmd(cmd, temp_dir).await?;
    }

    // 5. output_file_mappingに従ってファイルをPUT
    if let Some(output_mapping) = &cmd_config.output_file_mapping {
        for (path, url) in output_mapping {
            let source_path = temp_dir.join(path);
            if path.ends_with('/') {
                upload_directory_as_zip(url, &source_path, temp_dir).await?;
            } else {
                upload_file(url, &source_path, temp_dir).await?;
            }
        }
    }

    Ok(())
}
