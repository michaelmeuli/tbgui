use crate::config::TbguiConfig;
use crate::types::{LoadError, RemoteState};
use crate::utils::*;
use crate::{DEFAULT_TEMPLATE_FILENAME_LOCAL, RESULT_DIR_LOCAL};
use async_ssh2_tokio::client::{AuthMethod, Client, ServerCheckMethod};
use directories_next::UserDirs;
use rfd::FileDialog;
use russh_sftp::client::fs::ReadDir;
use russh_sftp::client::SftpSession;
use std::fs;
use std::path::PathBuf;
use tokio::fs::create_dir_all;

pub async fn create_client(config: &TbguiConfig) -> Result<Client, LoadError> {
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
    .await
    .map_err(|e| {
        println!("Failed to connect to the server: {}", e);
        let error = format!("{}", e);
        log_error(error.as_str());
        LoadError { error }
    })?;
    Ok(client)
}

pub async fn get_raw_reads(
    client: &Client,
    config: &TbguiConfig,
) -> Result<RemoteState, LoadError> {
    let remote_raw_dir: &str = config.remote_raw_dir.as_str();
    check_if_dir_exists(&client, &remote_raw_dir).await?;
    let command = format!("ls {}", remote_raw_dir);
    let result = client.execute(&command).await.map_err(|e| {
        log_error(&format!(
            "Failed to list files in remote directory: {:?}",
            e
        ));
        LoadError {
            error: format!("Failed to list files in remote directory: {:?}", e),
        }
    })?;
    assert_eq!(result.exit_status, 0);
    let stdout = result.stdout;

    let raw_reads: Vec<String> = stdout.lines().map(String::from).collect();
    let tasks = create_tasks(raw_reads);
    Ok(RemoteState { items: tasks })
}

pub async fn run_tbprofiler(
    client: &Client,
    items_checked: usize,
    samples: String,
    config: &TbguiConfig,
) -> Result<(), async_ssh2_tokio::Error> {
    let command_run_tbprofiler = format!(
        "sbatch --array 0-{} {} \"{}\"",
        items_checked - 1,
        config.tb_profiler_script.as_str(),
        samples
    );
    println!(
        "Running command_run_tbprofiler: {:?}",
        command_run_tbprofiler
    );
    let commandexecutedresult_run_tbprofiler = client.execute(&command_run_tbprofiler).await?;
    println!(
        "command_checkdir executed: {:?}",
        commandexecutedresult_run_tbprofiler
    );
    Ok(())
}

pub async fn download_results(
    client: &Client,
    config: &TbguiConfig,
) -> Result<(), async_ssh2_tokio::Error> {
    let channel = client.get_channel().await?;
    channel.request_subsystem(true, "sftp").await?;
    let sftp = SftpSession::new(channel.into_stream()).await?;

    let remote_dir = config.remote_results_dir.as_str();
    let local_dir: Option<PathBuf> = FileDialog::new()
        .set_title("Select directory to download results")
        .set_directory(UserDirs::new().unwrap().home_dir().join(RESULT_DIR_LOCAL))
        .pick_folder();
    let local_dir = match local_dir {
        Some(dir) => dir,
        None => {
            println!("No directory selected. Download canceled.");
            return Ok(());
        }
    };

    // Check if the remote directory exists
    match sftp.metadata(remote_dir).await {
        Ok(metadata) => {
            if !metadata.is_dir() {
                println!("Remote path exists but is not a directory: {}", remote_dir);
                return Err(async_ssh2_tokio::Error::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Specified remote path is not a directory",
                )));
            }
        }
        Err(e) => {
            println!(
                "Remote directory does not exist: {}. Error: {:?}",
                remote_dir, e
            );
            return Err(async_ssh2_tokio::Error::from(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("No such directory: {:?}", remote_dir),
            )));
        }
    }

    create_dir_all(local_dir.clone()).await?;
    let entries: ReadDir = sftp.read_dir(remote_dir).await?;

    for entry in entries {
        let file_name = entry.file_name();
        let file_type = entry.file_type();
        //let metadata = entry.metadata();
        println!("File: {}", file_name);
        let remote_file_path = format!("{}/{}", remote_dir, file_name);
        let local_file_path = local_dir.join(&file_name).clone();

        if file_type.is_file() && (file_name).ends_with(".docx") {
            if let Err(e) = download_file(&sftp, &remote_file_path, &local_file_path).await {
                println!("Error downloading file: {:?}", e);
            }
        }
    }
    Ok(())
}

pub async fn delete_results(
    client: &Client,
    config: &TbguiConfig,
) -> Result<(), async_ssh2_tokio::Error> {
    let command_checkdir = format!("ls {}/", config.remote_results_dir.as_str());
    println!("Running command_checkdir: {:?}", command_checkdir);
    let commandexecutedresult_checkdir = client.execute(&command_checkdir).await?;
    println!(
        "command_checkdir executed: {:?}",
        commandexecutedresult_checkdir
    );
    if commandexecutedresult_checkdir.exit_status != 0 {
        return Err(async_ssh2_tokio::Error::from(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("No such directory: {:?}", config.remote_results_dir),
        )));
    }
    let command_rm = format!("rm {}/*", config.remote_results_dir.as_str());
    println!("Running command_rm: {}", command_rm);
    let commandexecutedresult_rm = client.execute(&command_rm).await?;
    println!("command_rm executed: {:?}", commandexecutedresult_rm);
    let directory = UserDirs::new().unwrap().home_dir().join(RESULT_DIR_LOCAL);
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

pub async fn download_default_template(
    client: &Client,
    config: &TbguiConfig,
) -> Result<(), async_ssh2_tokio::Error> {
    let remote_file_path = config.default_template_remote.as_str();
    let save_directory: Option<PathBuf> = FileDialog::new()
        .set_title("Select directory to save template")
        .set_directory(UserDirs::new().unwrap().home_dir())
        .pick_folder();
    let save_directory = match save_directory {
        Some(dir) => dir,
        None => {
            println!("No directory selected. Download canceled.");
            return Ok(());
        }
    };
    let file_name: Option<String> = FileDialog::new()
        .set_title("Enter Filename for the Template")
        .set_file_name(DEFAULT_TEMPLATE_FILENAME_LOCAL)
        .save_file()
        .and_then(|path| {
            path.file_name()
                .map(|name| name.to_string_lossy().to_string())
        });
    let file_name = match file_name {
        Some(name) => name,
        None => {
            println!("No filename specified. Download canceled.");
            return Ok(());
        }
    };
    let local_file_path = save_directory.join(file_name);

    let channel = client.get_channel().await?;
    channel.request_subsystem(true, "sftp").await?;
    let sftp = SftpSession::new(channel.into_stream()).await?;

    if let Err(e) = download_file(&sftp, remote_file_path, &local_file_path).await {
        println!("Error downloading file: {:?}", e);
    } else {
        println!("File successfully downloaded to: {:?}", local_file_path);
    }
    Ok(())
}

pub async fn upload_user_template(
    client: &Client,
    config: &TbguiConfig,
) -> Result<(), async_ssh2_tokio::Error> {
    let remote_file_path = config.user_template_remote.as_str();
    let local_file_path: Option<PathBuf> = FileDialog::new()
        .set_title("Select File to Upload")
        .set_directory(UserDirs::new().unwrap().home_dir())
        .pick_file();
    let local_file_path = match local_file_path {
        Some(path) => path,
        None => {
            println!("No file selected. Upload canceled.");
            return Ok(());
        }
    };

    let channel = client.get_channel().await?;
    channel.request_subsystem(true, "sftp").await?;
    if let Err(e) = client.upload_file(local_file_path, remote_file_path).await {
        println!("Error uploading file: {:?}", e);
    }
    Ok(())
}
