pub mod schedule {
    use chrono::{DateTime, Datelike, Local, LocalResult, TimeZone};
    use std::time::Duration;

    fn zip_and_ship() {
        // ZIP N SHIP
        println!("Performing daily dispatch.");
    }

    pub fn checkpoint() {
        // Basically everyday at 6pm, gather all the files in the files folder, zip em and ship em

        let now: DateTime<Local> = Local::now();
        let six_pm_today: DateTime<Local> =
            match Local.with_ymd_and_hms(now.year(), now.month(), now.day(), 18, 0, 0) {
                LocalResult::Single(dt) => dt,
                LocalResult::None => {
                    // Time doesn't exist, e.g., due to daylight saving time changes.
                    panic!("Invalid time");
                }
                LocalResult::Ambiguous(_, _) => {
                    // Time is ambiguous, e.g., due to daylight saving time changes.
                    panic!("Ambiguous time");
                }
            };

        let duration = if now <= six_pm_today {
            six_pm_today - now
        } else {
            six_pm_today + chrono::Duration::days(1) - now
        };

        let duration_secs: u64 = duration.num_seconds() as u64;
        let duration = Duration::from_secs(duration_secs);

        // Sleep until 6 PM
        std::thread::sleep(duration);

        zip_and_ship();
    }
}
