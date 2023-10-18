use std::{fs::create_dir_all, path::Path};

use crate::get_dirs::get_dirs::{get_base, get_files, get_logs, get_root};
use crate::multithreading::run_threads;
use crate::update_log::update_log::{append_log, create_log};

mod get_dirs;
mod helpers;
mod multithreading;
mod update_log;
mod watch_file;
mod watch_folder;

fn main() {
    let mut files_to_watch: Vec<String> = Vec::new();
    println!("Starting Hyperlink.");

    let logs_binding: String = get_logs();
    let logs_path: &Path = Path::new(&logs_binding);
    let files_binding: String = get_files();
    let files_path: &Path = Path::new(&files_binding);

    if !Path::new(&logs_path).exists() {
        println!("No logs directory found. Creating...");
        _ = create_dir_all(logs_path);
        println!("Logs directory created successfully");
        println!(" ");
    };

    if !Path::new(&files_path).exists() {
        println!("No files directory found. Creating...");
        _ = create_dir_all(files_path);
        println!("Files directory created successfully");
        println!(" ");
    };

    _ = create_log();
    _ = append_log("Root folders created successfully. \n");

    // files_to_watch.push(format!("{}/other_config.txt", get_base()));
    // files_to_watch.push(format!("{}/test.txt", get_base()));

    // VSCode Keybindings
    files_to_watch.push(format!(
        "{}/AppData/Roaming/Code/User/keybindings.json",
        get_root()
    ));

    run_threads(files_to_watch);
    // Run threads for folders - use watch_folder::start_folder_watch;
}
