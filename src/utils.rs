use crate::types::Item;
use crate::RESULT_DIR_LOCAL;
use directories_next::UserDirs;
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use uuid::Uuid;

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
