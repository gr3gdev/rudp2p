#[allow(unused_imports)]
use std::thread;
#[allow(unused_imports)]
use std::time::Duration;

#[allow(unused_imports)]
use crate::server::{Message, Server, ServerStatus, Udp};
#[allow(unused_imports)]
use crate::server::Event;
#[allow(unused_imports)]
use crate::utils::ThreadSafe;

#[allow(dead_code)]
struct Test {
    events: ThreadSafe<Vec<Event>>,
}

#[allow(dead_code)]
impl Test {
    fn not_ready(&self) -> bool {
        let shared_events = self.events.clone();
        let guard = shared_events.lock().unwrap();
        guard.is_empty()
    }
}

#[allow(dead_code)]
struct TestMessage {
    content: String,
}

#[allow(dead_code)]
impl Message for TestMessage {
    fn content(&self) -> Vec<u8> {
        self.content.as_bytes().to_vec()
    }
}

#[test]
fn test_udp() {
    let test = Test {
        events: ThreadSafe::new(Vec::new())
    };

    let mut server1 = Udp::new(9001);
    server1.start();

    let mut server2 = Udp::new(9002);
    let shared_events = test.events.clone();
    server2.set_on_received(move |event, _socket| {
        let mut guard = shared_events.lock().unwrap();
        guard.push(event.clone());
    });
    server2.start();

    server1.send(TestMessage {
        content: "Hello I am Server1".to_string()
    }, &server2.addr());

    while test.not_ready() {
        thread::sleep(Duration::from_millis(100));
    }

    server1.stop();
    server2.stop();

    let shared_events = test.events.clone();
    let guard = shared_events.lock().unwrap();
    assert_eq!(1, guard.len());
    assert_eq!("Hello I am Server1".as_bytes().to_vec(), guard.get(0).unwrap().content);
    assert_eq!(server1.addr(), guard.get(0).unwrap().sender);

    while server1.alive() && server2.alive() {
        thread::sleep(Duration::from_millis(100));
    }
}