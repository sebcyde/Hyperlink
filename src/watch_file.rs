use notify::{
    Config, Event, ReadDirectoryChangesWatcher, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::fs::copy;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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
                println!("Inside Loop Match");
                let event: Event = event.unwrap();
                let content: String = format!("File change occurred: {:?}", event.kind);
                println!("File Change Occured: {}", content);
                _ = append_log(&content);

                match event.kind {
                    notify::EventKind::Modify(notify::event::ModifyKind::Any) => {
                        // Wait for a bit to ensure the file modification is complete.
                        let delay = Duration::from_millis(500);
                        thread::sleep(delay);

                        let files_path: String = get_files();
                        let file_name: &str = path.file_name().unwrap().to_str().unwrap();
                        let destination: String = format!("{}/{}", files_path, file_name);

                        // Perform the copy operation after waiting
                        _ = copy(&path, Path::new(&destination));
                        _ = append_log("Linked file updated. \n");

                        sender.send(format!("File {} changed", file_path)).unwrap();
                    }
                    _ => {}
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
