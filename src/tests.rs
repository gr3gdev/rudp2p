#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    use rudp2plib::peer::{Exchange, Peer};
    use rudp2plib::peer::message::PeerMessage;
    use rudp2plib::server::{Event, Server, ServerStatus, Udp};

    fn wait_while_condition(condition: &dyn Fn() -> bool) {
        while condition() {
            println!("waiting...");
            thread::sleep(Duration::from_millis(5));
        }
    }

    #[test]
    fn server_send() {
        let mut server1 = Udp::new(9001);
        let mut server2 = Udp::new(9002);

        let data: Arc<Mutex<Vec<Event>>> = Arc::new(Mutex::new(Vec::new()));
        let shared_data1 = Arc::clone(&data);
        let shared_data2 = Arc::clone(&data);

        server1.set_observer(move |e: &Event, _socket| {
            shared_data1.lock().unwrap().push(e.clone());
        });
        server1.set_on_started(move |a| {
            println!("[Server1] started : {}", a);
        });
        server1.set_on_stopped(move |a| {
            println!("[Server1] stopped : {}", a);
        });
        server2.set_observer(move |e: &Event, _socket| {
            shared_data2.lock().unwrap().push(e.clone());
        });
        server2.set_on_started(move |a| {
            println!("[Server2] started : {}", a);
        });
        server2.set_on_stopped(move |a| {
            println!("[Server2] stopped : {}", a);
        });

        server1.start();
        server2.start();

        server2.send("Hello I am Server2".as_bytes(), &server1.addr());

        wait_while_condition(&|| data.lock().unwrap().is_empty());

        server1.close();
        server2.close();

        let event = &data.lock().unwrap()[0];
        assert_eq!("Hello I am Server2", String::from_utf8(event.content.clone()).unwrap());
        assert_eq!(server2.addr(), event.sender);

        wait_while_condition(&|| server1.alive() || server2.alive());
    }

    #[test]
    fn peer_exchange_text() {
        let mut dispatcher = Peer::new(9100);
        dispatcher.start();

        let peers1: Arc<Mutex<Vec<SocketAddr>>> = Arc::new(Mutex::new(Vec::new()));
        let shared_peers1 = Arc::clone(&peers1);

        let data1: Arc<Mutex<Vec<PeerMessage>>> = Arc::new(Mutex::new(Vec::new()));
        let shared_data1 = Arc::clone(&data1);

        let data2: Arc<Mutex<Vec<PeerMessage>>> = Arc::new(Mutex::new(Vec::new()));
        let shared_data2 = Arc::clone(&data2);

        let data3: Arc<Mutex<Vec<PeerMessage>>> = Arc::new(Mutex::new(Vec::new()));
        let shared_data3 = Arc::clone(&data3);

        let mut peer1 = Peer::new(9101);
        let mut peer2 = Peer::new(9102);
        let mut peer3 = Peer::new(9103);

        peer1.connect(&dispatcher);
        peer1.set_on_peer_connected(move |a: &SocketAddr| {
            println!("[PEER1] connected with {}", a);
            shared_peers1.lock().unwrap().push(a.clone());
        });
        peer1.set_on_message_received(move |e: &PeerMessage| {
            shared_data1.lock().unwrap().push(e.clone());
        });

        peer2.connect(&dispatcher);
        peer2.set_on_peer_connected(move |a: &SocketAddr| {
            println!("[PEER2] connected with {}", a);
        });
        peer2.set_on_message_received(move |e: &PeerMessage| {
            shared_data2.lock().unwrap().push(e.clone());
        });

        peer3.connect(&dispatcher);
        peer3.set_on_peer_connected(move |a: &SocketAddr| {
            println!("[PEER3] connected with {}", a);
        });
        peer3.set_on_message_received(move |e: &PeerMessage| {
            shared_data3.lock().unwrap().push(e.clone());
        });

        wait_while_condition(&|| peers1.lock().unwrap().len() < 2);

        let peer_message = PeerMessage::new(Vec::from("Hello everybody".as_bytes()));
        let uid = PeerMessage::uid(&peer_message);
        peer1.send_to_all(peer_message);

        wait_while_condition(&|| data2.lock().unwrap().is_empty() || data3.lock().unwrap().is_empty());

        peer1.close();
        peer2.close();
        peer3.close();
        dispatcher.close();

        let message2 = &data2.lock().unwrap()[0];
        assert_eq!("Hello everybody", message2.to_string());
        assert_eq!(uid, PeerMessage::uid(message2));

        let message3 = &data3.lock().unwrap()[0];
        assert_eq!("Hello everybody", message3.to_string());
        assert_eq!(uid, PeerMessage::uid(message3));

        wait_while_condition(&|| dispatcher.alive() || peer1.alive() || peer2.alive() || peer3.alive());
    }
}