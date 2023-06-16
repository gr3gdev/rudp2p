use std::fmt::Display;

pub(crate) struct Logger;

impl Logger {
    pub(crate) fn info(message: &str) {
        println!("\x1b[32m[INFO]\x1b[0m {message}");
    }

    pub(crate) fn warn(message: &str) {
        println!("\x1b[33m[WARN]\x1b[0m {message}");
    }

    pub(crate) fn error<E>(e: E)
    where
        E: Display,
    {
        println!("\x1b[31m[ERROR]\x1b[0m {e}");
    }
}
