use crate::config::TbguiConfig;
use crate::types::Item;
use crate::types::LoadError;
use crate::RESULT_DIR_LOCAL;
use async_ssh2_tokio::client::Client;
use directories_next::UserDirs;
use russh_sftp::{client::SftpSession, protocol::OpenFlags};
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

pub async fn download_file(
    sftp: &SftpSession,
    remote_file_path: &str,
    local_file_path: &PathBuf,
) -> Result<(), async_ssh2_tokio::Error> {
    println!("Downloading: {}", remote_file_path);
    let mut remote_file = sftp
        .open_with_flags(remote_file_path, OpenFlags::READ)
        .await?;
    if let Some(parent) = Path::new(local_file_path).parent() {
        create_dir_all(parent).await?;
    }
    let mut local_file = File::create(local_file_path.clone()).await?;
    let mut buffer = [0u8; 4096];

    loop {
        let n = remote_file.read(&mut buffer).await?;
        if n == 0 {
            break; // End of file
        }
        local_file.write_all(&buffer[..n]).await?;
    }
    println!("File downloaded successfully to {:?}", local_file_path);
    Ok(())
}

pub async fn check_if_running(
    client: &Client,
    config: &TbguiConfig,
) -> Result<bool, async_ssh2_tokio::Error> {
    let command_check_running = format!("squeue -u {}", config.username.as_str());
    let commandexecutedresult_check_if_running = client.execute(&command_check_running).await?;
    let running = commandexecutedresult_check_if_running
        .stdout
        .contains(config.username.as_str());
    Ok(running)
}

pub fn create_tasks(reads: Vec<String>) -> Vec<Item> {
    let mut tasks = Vec::new();
    let mut seen_samples = HashSet::new();

    for file_name in reads {
        if let Some((sample, _suffix)) = file_name.split_once('_') {
            if seen_samples.insert(sample.to_string()) {
                tasks.push(Item {
                    id: Uuid::new_v4(),
                    sample: sample.to_string(),
                    is_checked: false,
                });
            }
        }
    }
    tasks
}

pub async fn check_if_dir_exists(client: &Client, remote_raw_dir: &str) -> Result<(), LoadError> {
    let command = format!("test -d {} && echo 'exists'", remote_raw_dir);
    let result = client.execute(&command).await.map_err(|e| {
        log_error(&format!(
            "Failed to check if remote directory exists: {:?}",
            e
        ));
        LoadError {
            error: format!("Failed to check if remote directory exists: {:?}", e),
        }
    })?;
    if result.stdout.trim() != "exists" {
        log_error(&format!(
            "Remote directory does not exist: {:?}",
            remote_raw_dir
        ));
        return Err(LoadError {
            error: format!("Remote directory does not exist: {:?}", remote_raw_dir),
        });
    }
    else {
        Ok(())
    }
}
pub fn log_error(message: &str) {
    let error_file = UserDirs::new()
        .unwrap()
        .home_dir()
        .join(RESULT_DIR_LOCAL)
        .join("error.log");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(error_file)
        .expect("Failed to open log file");
    writeln!(file, "{}", message).expect("Failed to write to log file");
}

pub fn delete_log_file() {
    let error_file = UserDirs::new()
        .unwrap()
        .home_dir()
        .join(RESULT_DIR_LOCAL)
        .join("error.log");
    println!("Attempting to delete: {:?}", error_file);
    if fs::remove_file(&error_file).is_ok() {
        println!("File {:?} deleted successfully.", error_file);
    } else {
        println!(
            "Failed to delete the file {:?}. It may not exist.",
            error_file
        );
    }
}
