extern crate notify;

use notify::{
    Config, EventKind, ReadDirectoryChangesWatcher, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::{get_dirs::get_dirs::get_root, update_log::update_log::append_log};

pub fn watch_folder<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Create new watcher
    let mut watcher: ReadDirectoryChangesWatcher = RecommendedWatcher::new(tx, Config::default())?;

    // Path to be watched - Non-Recursive Mode
    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                let event_path: PathBuf = event.paths.get(0).unwrap().to_owned();
                println!("Update Type: {:?}", event.kind);

                // Functions for each event type
                match event.kind {
                    EventKind::Remove(_) => {}
                    EventKind::Access(_) => {}
                    EventKind::Any => {}
                    EventKind::Other => {}
                    EventKind::Modify(_) | EventKind::Create(_) => {
                        let event_path_clone: &'static PathBuf =
                            Box::leak(Box::new(event_path.clone()));
                        let target: &OsStr = Path::file_name(&event_path_clone).unwrap();

                        let event_path_str: &str = event_path_clone.to_str().unwrap();

                        let event_kind_str: &str = "Modify";
                    }
                };

                println!(" ");
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

pub fn start_folder_watch(dir_name: &'static str) {
    let dir_path: PathBuf = Path::new(get_root().as_str()).join(dir_name);
    println!("Starting {} watcher", dir_name);
    _ = append_log(&format!("Starting {} watcher.", dir_name));
    _ = watch_folder(&dir_path);
}
