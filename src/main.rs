use std::{fs::create_dir_all, path::Path};

use crate::get_dirs::get_dirs::{get_base, get_files, get_logs};
use crate::update_log::update_log::{append_log, create_log};

mod get_dirs;
mod helpers;
mod update_log;
mod watch_file;
mod watch_folder;

fn main() {
    use watch_file::start_file_watch;
    // use watch_folder::start_folder_watch;

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

    let vs_keybindings: String = format!("{}/test.txt", get_base());

    _ = start_file_watch(vs_keybindings);
}
