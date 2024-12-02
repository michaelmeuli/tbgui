use directories_next::UserDirs;
use std::fs::OpenOptions;
use std::io::Write;
use std::collections::HashMap;
use crate::{Item, USERNAME, REMOTE_RAW_DIR};
use uuid::Uuid;
use async_ssh2_tokio::client::{AuthMethod, Client, ServerCheckMethod};


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

pub async fn get_file_names(client: &Client) -> Result<Vec<String>, async_ssh2_tokio::Error> {
    let command = format!("test -d {} && echo 'exists'", REMOTE_RAW_DIR);
    let result = client.execute(&command).await?;
    if result.stdout.trim() != "exists" {
        log_error(&format!("Directory on remote with raw reads does not exist: {}", REMOTE_RAW_DIR));
        panic!("Directory on remote with raw reads does not exist: {}", REMOTE_RAW_DIR);
    }

    let command = format!("ls {}", REMOTE_RAW_DIR);
    let result = client.execute(&command).await?;
    assert_eq!(result.exit_status, 0);
    let stdout = result.stdout;

    let file_names: Vec<String> = stdout.lines().map(String::from).collect();
    Ok(file_names)
}

pub fn create_tasks(file_names: Vec<String>) -> Vec<Item> {
    let mut tasks = Vec::new();
    let mut grouped_files: HashMap<String, (String, String)> = HashMap::new();
    for file_name in file_names {
        if let Some((sample, suffix)) = file_name.split_once('_') {
            match suffix {
                "1.fastq.gz" => {
                    grouped_files
                        .entry(sample.to_string())
                        .or_insert_with(|| (String::new(), String::new()))
                        .0 = file_name.to_string();
                }
                "2.fastq.gz" => {
                    grouped_files
                        .entry(sample.to_string())
                        .or_insert_with(|| (String::new(), String::new()))
                        .1 = file_name.to_string();
                }
                _ => {}
            }
        }
    }
    for (sample, (read1, read2)) in grouped_files {
        tasks.push(Item {
            id: Uuid::new_v4(),
            sample,
            read1,
            read2,
            is_checked: false,
        });
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