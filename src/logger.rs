use std::time::{SystemTime, UNIX_EPOCH};

fn timestamp() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => format!("{}", d.as_secs()),
        Err(_) => "0".into(),
    }
}

pub fn log_error(context: &str, details: &str) {
    eprintln!("[ERROR] {} [{}]: {}", context, timestamp(), details);
}

pub fn log_warn(context: &str, details: &str) {
    eprintln!("[WARN] {} [{}]: {}", context, timestamp(), details);
}

pub fn log_info(context: &str, details: &str) {
    eprintln!("[INFO] {} [{}]: {}", context, timestamp(), details);
}
