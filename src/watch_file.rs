use notify::{
    Config, Event, ReadDirectoryChangesWatcher, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::fs::copy;
use std::path::{Path, PathBuf};
use std::sync::mpsc;

use crate::get_dirs::get_dirs::get_files;
use crate::update_log::update_log::append_log;

pub fn watch_file(file_path: String, sender: mpsc::Sender<String>) -> notify::Result<()> {
    let path: PathBuf = Path::new(&file_path).to_owned();

    println!("Watching {:?}.", &path);
    _ = append_log(&format!("Watching {:?}.", path));

    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher: ReadDirectoryChangesWatcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(event) => {
                let event: Event = event.unwrap();
                let content: String = format!("File change occurred: {:?}", event.kind);
                _ = append_log(&content);

                let files_path: String = get_files();
                let file_name: &str = path.file_name().unwrap().to_str().unwrap();
                let destination: String = format!("{}/{}", files_path, file_name);

                _ = copy(&path, Path::new(&destination));
                _ = append_log("Linked file updated. \n");

                sender.send(format!("File {} changed", file_path)).unwrap();
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
