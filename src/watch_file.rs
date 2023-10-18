use notify::{Config, ReadDirectoryChangesWatcher, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::copy;
use std::path::{Path, PathBuf};

use crate::get_dirs::get_dirs::get_files;
use crate::update_log::update_log::append_log;

fn watch_file(path: &Path) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher: ReadDirectoryChangesWatcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(event) => event_handler(event.unwrap(), &path),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn event_handler(event: notify::Event, path: &Path) {
    // println!("File change occurred: {:?}", event);
    let content: String = format!("File change occurred: {:?}", event.kind);
    _ = append_log(&content);

    let files_path: String = get_files();
    let file_name: &str = path.file_name().unwrap().to_str().unwrap();
    let destination: String = format!("{}/{}", files_path, file_name);

    _ = copy(path, Path::new(&destination));
    _ = append_log("Linked file updated. \n");
}

pub fn start_file_watch(file_path: String) {
    let full_file_path: PathBuf = Path::new(&file_path).to_owned();
    println!("Watching {:?}.", &full_file_path);
    _ = append_log(&format!("Watching {:?}.", full_file_path));
    _ = watch_file(&full_file_path);
}
