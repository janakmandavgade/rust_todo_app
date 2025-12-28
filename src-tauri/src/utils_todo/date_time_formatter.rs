use chrono::Local;

pub fn get_current_date_time() -> String {
    return Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
}