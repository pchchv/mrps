use chrono::{Local, DateTime};

pub fn time_string (time: DateTime<Local>) -> String {
    time.format("%Y-%m-%d %H:%M:%S").to_string()
}
