use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::{Arc, LockResult, Mutex, MutexGuard};

// Arc : permet de partager la liste entre plusieurs threads
// Mutex : permet de verrouiller la liste pour garantir l'accès exclusif aux threads

// COMMON FUNCTIONS

/// Read a file into Vec<u8> from a path.
pub fn read_file(path: &str) -> Vec<u8> {
    let mut f = File::open(path).expect(format!("File not found : {}", path).as_str());
    let metadata = fs::metadata(&path).expect("Unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("Buffer overflow");
    buffer
}

/// Write a file into a path from [u8].
pub fn write_file(data: &[u8], path: &str) -> File {
    let mut file = File::create(path).expect(format!("Unable to create the file {}", path).as_str());
    file.write(data).expect("Unable to write into file");
    file
}

// STRUCT

/// ThreadSafe data, use Arc and Mutex.
pub struct ThreadSafe<T> {
    /// ThreadSafe data.
    data: Arc<Mutex<T>>,
}

pub struct OptionalClosure<T: ?Sized> {
    /// Optional closure.
    data: ThreadSafe<RefCell<Option<Box<T>>>>,
}

// IMPL

impl<T> ThreadSafe<T> {
    pub fn new(val: T) -> ThreadSafe<T> {
        ThreadSafe {
            data: Arc::new(Mutex::new(val))
        }
    }

    pub fn clone(&self) -> Arc<Mutex<T>> {
        Arc::clone(&self.data)
    }

    pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
        self.data.lock()
    }
}

impl<T: ?Sized> OptionalClosure<T> {
    pub fn new(val: Option<Box<T>>) -> OptionalClosure<T> {
        OptionalClosure {
            data: ThreadSafe::new(RefCell::new(val))
        }
    }

    pub fn set(data: &OptionalClosure<T>, closure: Box<T>) {
        let closure = Some(closure);
        let closure_cell = RefCell::new(closure);
        let closure_mutex = data.data.clone();
        let mut guard = closure_mutex.lock().unwrap();
        *guard = closure_cell;
    }

    pub fn shared(&self) -> Arc<Mutex<RefCell<Option<Box<T>>>>> {
        self.data.clone()
    }
}

impl<T: Debug> Debug for ThreadSafe<T> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
