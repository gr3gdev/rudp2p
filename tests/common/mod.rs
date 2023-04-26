#[cfg(test)]
use std::thread;
use std::time::{Duration, SystemTime};

pub(crate) fn wait_while_condition(condition: &dyn Fn() -> bool) {
    let start = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap().as_millis();
    while condition() {
        thread::sleep(Duration::from_millis(1000));
        let current = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap().as_millis();
        if current - start > 5000 {
            panic!("Timeout !");
        }
    }
}
