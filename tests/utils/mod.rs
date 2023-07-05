use std::fs::File;
use std::io::Read;
use std::time::{Duration, SystemTime};
use std::{env, fs, thread};

pub(crate) fn read_file(file: &str) -> Vec<u8> {
    let current_dir = env::current_dir().unwrap();
    let mut path = current_dir.display().to_string();
    path.push_str(file.trim());
    let mut f = File::open(&path).expect(format!("File not found : {}", path).as_str());
    let metadata = fs::metadata(&path).expect("Unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("Buffer overflow");
    buffer
}

pub(crate) fn wait_while_condition(condition: &dyn Fn() -> bool) {
    let start = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    while condition() {
        thread::sleep(Duration::from_millis(1000));
        let current = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        if current - start > 5000 {
            panic!("Timeout !");
        }
    }
}
