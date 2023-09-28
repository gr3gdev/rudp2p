use std::fs::File;
use std::io::Read;
use std::time::{SystemTime, Duration};
use std::{env, fs};

use futures::Future;

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

pub(crate) fn get_time() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub(crate) async fn wait_until<F, Fut>(condition: F, timeout_ms: u128) -> bool
where
    F: Fn() -> Fut,
    Fut: Future<Output = bool>,
{
    let start = get_time();
    let mut res;
    loop {
        std::thread::sleep(Duration::from_millis(100));
        res = condition().await;
        if res || get_time() - start > timeout_ms {
            break;
        }
    }
    res
}
