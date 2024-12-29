use crate::types::{Item, LoadError, RemoteState};
use crate::{
    TB_PROFILER_SCRIPT, USER_TEMPLATE_REMOTE
};
use crate::config::TbguiConfig;
use async_ssh2_tokio::client::{AuthMethod, Client, ServerCheckMethod};
use directories_next::UserDirs;
use russh_sftp::client::fs::ReadDir;
use russh_sftp::{client::SftpSession, protocol::OpenFlags};
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

pub async fn create_client(config: &TbguiConfig) -> Result<Client, async_ssh2_tokio::Error> {
    let key_path = UserDirs::new()
        .unwrap()
        .home_dir()
        .join(".ssh")
        .join("id_rsa");
    let auth_method = AuthMethod::with_key_file(key_path, None);
    let client = Client::connect(
        ("130.60.24.133", 22),
        config.username.as_str(),
        auth_method,
        ServerCheckMethod::NoCheck,
    )
    .await?;
    Ok(client)
}

pub async fn get_raw_reads(client: &Client, config: &TbguiConfig) -> Result<Vec<String>, async_ssh2_tokio::Error> {
    let remote_raw_dir: &str = config.remote_raw_dir.as_str();
    let command = format!("test -d {} && echo 'exists'", remote_raw_dir);
    let result = client.execute(&command).await?;
    if result.stdout.trim() != "exists" {
        log_error(&format!(
            "Directory on remote with raw reads does not exist: {}",
            remote_raw_dir
        ));
        panic!(
            "Directory on remote with raw reads does not exist: {}",
            remote_raw_dir
        );
    }

    let command = format!("ls {}", remote_raw_dir);
    let result = client.execute(&command).await?;
    assert_eq!(result.exit_status, 0);
    let stdout = result.stdout;

    let raw_reads: Vec<String> = stdout.lines().map(String::from).collect();
    Ok(raw_reads)
}

pub async fn run_tbprofiler(
    client: &Client,
    items_checked: usize,
    samples: String,
) -> Result<(), async_ssh2_tokio::Error> {
    let command = format!(
        "sbatch --array 0-{} {} \"{}\"",
        items_checked - 1,
        TB_PROFILER_SCRIPT,
        samples
    );
    println!("Running command: {}", command);
    client.execute(&command).await?;
    Ok(())
}

pub async fn download_results(client: &Client, config: &TbguiConfig) -> Result<(), async_ssh2_tokio::Error> {
    let channel = client.get_channel().await?;
    channel.request_subsystem(true, "sftp").await?;
    let sftp = SftpSession::new(channel.into_stream()).await?;

    let remote_dir = config.remote_results_dir.as_str();
    let local_dir = UserDirs::new()
        .unwrap()
        .home_dir()
        .join("tb-profiler-results");
    tokio::fs::create_dir_all(local_dir.clone()).await?;
    let entries: ReadDir = sftp.read_dir(remote_dir).await?;

    for entry in entries {
        let file_name = entry.file_name();
        let file_type = entry.file_type();
        //let metadata = entry.metadata();
        println!("File: {}", file_name);
        let remote_file_path = format!("{}/{}", remote_dir, file_name);
        let local_file_path = local_dir.join(&file_name).clone();

        if file_type.is_file() && (&file_name).ends_with(".docx") {
            if let Err(e) = download_file(&sftp, &remote_file_path, &local_file_path).await {
                println!("Error downloading file: {:?}", e);
            }
        }
    }
    Ok(())
}

pub async fn delete_results(client: &Client, config: &TbguiConfig) -> Result<(), async_ssh2_tokio::Error> {
    let command = format!("rm {}/*", config.remote_results_dir.as_str());
    client.execute(&command).await?;
    let directory = UserDirs::new()
        .unwrap()
        .home_dir()
        .join("tb-profiler-results");
    if !directory.is_dir() {
        println!("Directory does not exist: {:?}", directory);
        return Ok(());
    }
    for entry in fs::read_dir(&directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            fs::remove_file(&path)?;
            println!("Deleted file: {:?}", path);
        }
    }
    println!("All files in {:?} have been deleted.", directory);
    Ok(())
}

pub async fn download_default_template(client: &Client, config: &TbguiConfig) -> Result<(), async_ssh2_tokio::Error> {
    let remote_file_path = config.default_template_remote.as_str();
    let local_file_path = UserDirs::new()
        .unwrap()
        .home_dir()
        .join("tb-profiler-results")
        .join("default_template.docx");

    let channel = client.get_channel().await?;
    channel.request_subsystem(true, "sftp").await?;
    let sftp = SftpSession::new(channel.into_stream()).await?;

    if let Err(e) = download_file(&sftp, &remote_file_path, &local_file_path).await {
        println!("Error downloading file: {:?}", e);
    }
    Ok(())
}

pub async fn upload_user_template(client: &Client) -> Result<(), async_ssh2_tokio::Error> {
    let remote_file_path = USER_TEMPLATE_REMOTE;
    let local_file_path = UserDirs::new()
        .unwrap()
        .home_dir()
        .join("tb-profiler-results")
        .join("user_template.docx");

    let channel = client.get_channel().await?;
    channel.request_subsystem(true, "sftp").await?;
    if let Err(e) = client.upload_file(local_file_path, remote_file_path).await {
        println!("Error uploading file: {:?}", e);
    }
    Ok(())
}

pub async fn download_file(
    sftp: &SftpSession,
    remote_file_path: &str,
    local_file_path: &PathBuf,
) -> Result<(), async_ssh2_tokio::Error> {
    println!("Downloading: {}", remote_file_path);
    let mut remote_file = sftp
        .open_with_flags(remote_file_path, OpenFlags::READ)
        .await?;
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

pub async fn load(config: &TbguiConfig) -> Result<RemoteState, LoadError> {
    delete_log_file();
    match create_client(config).await {
        Ok(client) => {
            println!("Connected to the server");
            let reads = get_raw_reads(&client, config).await.map_err(|e| LoadError {
                error: e.to_string(),
            })?;

            let tasks = create_tasks(reads);
            Ok(RemoteState {
                client,
                items: tasks,
            })
        }
        Err(e) => {
            println!("Failed to connect to the server: {}", e);
            let error = format!("{}", e);
            log_error(error.as_str());
            Err(LoadError { error })
        }
    }
}

pub fn log_error(message: &str) {
    let error_file = UserDirs::new()
        .unwrap()
        .home_dir()
        .join("tb-profiler-results")
        .join("error.log");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(error_file)
        .expect("Failed to open log file");
    writeln!(file, "{}", message).expect("Failed to write to log file");
}

pub fn delete_log_file() {
    let error_file = UserDirs::new()
        .unwrap()
        .home_dir()
        .join("tb-profiler-results")
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
