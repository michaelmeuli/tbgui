use crate::{Item, REMOTE_RAW_DIR, TB_PROFILER_SCRIPT, USERNAME};
use async_ssh2_tokio::client::{AuthMethod, Client, ServerCheckMethod};
use directories_next::UserDirs;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;
use uuid::Uuid;

pub async fn create_client() -> Result<Client, async_ssh2_tokio::Error> {
    let key_path = match ssh_key_path() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get SSH key path: {}", e);
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

pub async fn run_tbprofiler(client: &Client, items_checked: usize, samples: String) -> Result<(), async_ssh2_tokio::Error> {
    let command = format!("sbatch --array 0-{} {} \\\"{}\\\"", items_checked-1, TB_PROFILER_SCRIPT, samples);
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

fn log_error(message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true) // Clear the file's contents
        .open("error.log")
        .expect("Failed to open log file");
    writeln!(file, "{}", message).expect("Failed to write to log file");
}
