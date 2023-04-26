pub(crate) struct Logger {}

impl Logger {
    pub(crate) fn info(message: String) {
        println!("\x1b[32m[INFO]\x1b[0m {}", message);
    }
}
