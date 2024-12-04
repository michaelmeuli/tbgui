use crate::{Item, REMOTE_RAW_DIR, TB_PROFILER_SCRIPT, USERNAME, REMOTE_RESULTS_DIR};
use async_ssh2_tokio::client::{AuthMethod, Client, ServerCheckMethod};
use directories_next::UserDirs;
use russh_sftp::client::fs::ReadDir;
use russh_sftp::{client::SftpSession, protocol::OpenFlags};
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use tokio::fs::File;
use uuid::Uuid;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

pub async fn create_client() -> Result<Client, async_ssh2_tokio::Error> {
    let key_path = match ssh_key_path() {
        Ok(path) => path,
        Err(_e) => {
            println!("create_client(): Failed to get SSH key path.");
            log_error("create_client(): Failed to get SSH key path.");
            return Err(async_ssh2_tokio::Error::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "SSH key path not found",
            )));
        }
    };

    let auth_method = AuthMethod::with_key_file(key_path, None);
    let client = Client::connect(
        ("130.60.24.133", 22),
        USERNAME,
        auth_method,
        ServerCheckMethod::NoCheck,
    )
    .await?;

    Ok(client)
}

pub async fn get_raw_reads(client: &Client) -> Result<Vec<String>, async_ssh2_tokio::Error> {
    let command = format!("test -d {} && echo 'exists'", REMOTE_RAW_DIR);
    let result = client.execute(&command).await?;
    if result.stdout.trim() != "exists" {
        log_error(&format!(
            "Directory on remote with raw reads does not exist: {}",
            REMOTE_RAW_DIR
        ));
        panic!(
            "Directory on remote with raw reads does not exist: {}",
            REMOTE_RAW_DIR
        );
    }

    let command = format!("ls {}", REMOTE_RAW_DIR);
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

pub fn ssh_key_path() -> Result<String, String> {
    if let Some(user_dirs) = UserDirs::new() {
        let path = user_dirs.home_dir().join(".ssh").join("id_rsa");
        if path.exists() {
            match path.to_str() {
                Some(path_str) => Ok(path_str.to_string()),
                None => Err("Failed to convert SSH key path to string".to_string()),
            }
        } else {
            Err(format!("SSH key file does not exist at: {:?}", path))
        }
    } else {
        Err("Failed to determine the user's home directory".to_string())
    }
}

pub async fn download_results(client: &Client) -> Result<(), async_ssh2_tokio::Error> {
    let channel = client.get_channel().await?;
    channel.request_subsystem(true, "sftp").await?;
    let sftp = SftpSession::new(channel.into_stream()).await?;

    let remote_dir = REMOTE_RESULTS_DIR;
    let local_dir = "./results";
    tokio::fs::create_dir_all(local_dir).await?;
    let entries: ReadDir = sftp.read_dir(remote_dir).await?;

    for entry in entries {
        let file_name = entry.file_name();
        let file_type = entry.file_type();
        let metadata = entry.metadata();

        println!("File: {}", file_name);
        println!("File Type: {:?}", file_type);
        println!("Metadata: {:?}", metadata);

        if file_type.is_file() && file_name.ends_with(".docx") {
            let remote_file_path = format!("{}/{}", remote_dir, file_name);
            let local_file_path = format!("{}/{}", local_dir, file_name);
            println!("Downloading: {}", remote_file_path);
            let mut remote_file = sftp
                .open_with_flags(&remote_file_path, OpenFlags::READ)
                .await?;
            let mut local_file = File::create(&local_file_path).await?;

            let mut buffer = [0u8; 4096];
            loop {
                let n = remote_file.read(&mut buffer).await?;
                if n == 0 {
                    break; // End of file
                }
                local_file.write_all(&buffer[..n]).await?;
            }
            println!("File downloaded successfully to {}", local_file_path);
        }
    }
    Ok(())
}

pub fn log_error(message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("error.log")
        .expect("Failed to open log file");
    writeln!(file, "{}", message).expect("Failed to write to log file");
}

pub fn delete_log_file() {
    let file_path = "error.log";
    if fs::remove_file(file_path).is_ok() {
        println!("File '{}' deleted successfully.", file_path);
    } else {
        println!("Failed to delete the file '{}'. It may not exist.", file_path);
    }
}