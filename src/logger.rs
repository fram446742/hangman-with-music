pub fn log_error(context: &str, details: &str) {
    eprintln!("{}: {}", context, details);
}
