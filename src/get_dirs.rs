pub mod get_dirs {
    use std::path::Path;

    fn is_at_work() -> bool {
        return Path::new("C:/Users/sebastian.cyde").exists();
    }

    pub fn get_base() -> String {
        if is_at_work() {
            return String::from("C:/Users/sebastian.cyde/Documents/Hyperlink");
        } else {
            return String::from("C:/Users/SebCy/Documents/Hyperlink");
        }
    }

    pub fn get_root() -> String {
        if is_at_work() {
            return String::from("C:/Users/sebastian.cyde");
        } else {
            return String::from("C:/Users/SebCy");
        }
    }

    pub fn get_logs() -> String {
        if is_at_work() {
            return String::from("C:/Users/sebastian.cyde/Documents/Hyperlink/Logs");
        } else {
            return String::from("C:/Users/SebCy/Documents/Hyperlink/Logs");
        }
    }

    pub fn get_files() -> String {
        if is_at_work() {
            return String::from("C:/Users/sebastian.cyde/Documents/Hyperlink/Files");
        } else {
            return String::from("C:/Users/SebCy/Documents/Hyperlink/Files");
        }
    }
}
