use std::{sync::mpsc::Sender, thread::JoinHandle};

use crate::{update_log::update_log::append_log, watch_file::watch_file};

// Set up threads and watchers for each of the specified files
pub fn run_threads(files_to_watch: Vec<String>) {
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();
    let mut threads: Vec<thread::JoinHandle<()>> = vec![];

    for path in files_to_watch {
        let tx: Sender<String> = tx.clone();
        let path: String = path.clone();

        let thread: JoinHandle<()> = thread::spawn(move || {
            _ = watch_file(path, tx);
        });

        threads.push(thread);
    }

    for event in rx {
        let content: String = format!("Event: {}", event);
        println!("Event: {}", event);
        _ = append_log(&content)
    }
}
