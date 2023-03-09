use std::cell::RefCell;
use std::sync::{Arc, Mutex};

// Arc : permet de partager la liste entre plusieurs threads
// Mutex : permet de verrouiller la liste pour garantir l'accès exclusif aux threads
pub type ThreadSafe<T> = Arc<Mutex<T>>;

pub fn new_thread_safe<T>(val: T) -> ThreadSafe<T> {
    Arc::new(Mutex::new(val))
}

pub fn get_safe<T>(data: &ThreadSafe<T>) -> Arc<Mutex<T>> {
    Arc::clone(data)
}

pub type OptionalClosure<T> = ThreadSafe<RefCell<Option<Box<T>>>>;

pub fn new_optional_closure<T: ?Sized>(val: Option<Box<T>>) -> OptionalClosure<T> {
    Arc::new(Mutex::new(RefCell::new(val)))
}

pub fn set_optional_closure<T: ?Sized>(data: &OptionalClosure<T>, closure: Box<T>) -> () {
    let closure = Some(closure);
    let closure_cell = RefCell::new(closure);
    let closure_mutex = Arc::clone(data);
    let mut guard = closure_mutex.lock().unwrap();
    *guard = closure_cell;
}

pub fn get_optional_shared<T: ?Sized>(data: &OptionalClosure<T>) -> Arc<Mutex<RefCell<Option<Box<T>>>>> {
    Arc::clone(data)
}
