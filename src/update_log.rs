pub mod update_log {

    use chrono::{DateTime, Utc};
    use std::fs::{File, OpenOptions};
    use std::io::{self, BufWriter, Write};
    use std::path::{Path, PathBuf};

    use crate::get_dirs::get_dirs::get_logs;
    use crate::helpers::helpers::get_dir_entries;

    pub fn get_current_time(format: &'static str) -> String {
        let utc: DateTime<Utc> = Utc::now();
        let formatted_dt: String = utc.format(format).to_string();
        return formatted_dt;
    }

    pub fn create_log() -> io::Result<()> {
        let file_name: String = format!("Hyperlink_{}.txt", get_current_time("%Y%m%d"));
        let full_log_path: PathBuf = Path::new(get_logs().as_str()).join(&file_name);

        if Path::new(&full_log_path).is_file() {
            _ = append_log("Ended Session\n");
            _ = append_log("Started new session");
            return Ok(());
        }

        println!("Creating log...");
        _ = File::create(&full_log_path);
        _ = append_log("Log Created.");

        println!(" ");

        return Ok(());
    }

    pub fn append_log(content: &str) -> Result<(), io::Error> {
        // println!("LOGGING: {}", content);
        let binding: String = get_logs();
        let log_path: &Path = Path::new(binding.as_str());

        let all_logs: Vec<PathBuf> = get_dir_entries((&log_path).to_path_buf());

        // Get the latest log and open it in append mode
        let latest_log: &PathBuf = all_logs.iter().max().unwrap();
        let current_time: String = get_current_time("%H:%M:%S");
        println!("Updating log at: {}", &current_time);

        let mut options: OpenOptions = OpenOptions::new();
        options.append(true);
        let log: File = options.open(latest_log)?;

        let mut writer: BufWriter<File> = BufWriter::new(log);

        let log_content_bytes: Vec<u8> = format!("{} - {}\n", current_time, content).into_bytes();
        let log_content: &[u8] = &log_content_bytes;

        writer.write_all(log_content)?;
        writer.flush()?;

        return Ok(());
    }
}
