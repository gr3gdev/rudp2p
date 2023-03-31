#[cfg(test)]
use std::thread;
use std::time::{Duration, SystemTime};

pub fn wait_while_condition(message: &str, condition: &dyn Fn() -> bool) {
    let start = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap().as_millis();
    while condition() {
        println!("Waiting {}...", message);
        thread::sleep(Duration::from_millis(10));
        let current = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap().as_millis();
        if current - start > 5000 {
            panic!("Timeout {} !", message);
        }
    }
}
