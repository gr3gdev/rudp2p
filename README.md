# RUDP2P
[![Rust tests](https://github.com/gr3gdev/rudp2p/actions/workflows/rust.yml/badge.svg)](https://github.com/gr3gdev/rudp2p/actions/workflows/rust.yml)
[![Rust Docs](https://img.shields.io/badge/docs.rs-rudp2p-green)](https://docs.rs/rudp2p/)

RUDP2P is a Rust library for P2P exchange with the UDP protocol.

## Example

```rust
use async_trait::async_trait;
use rudp2plib::{
    configuration::Configuration, 
    dao::InMemoryDao, 
    network::{Response, events::Message}, 
    observer::Observer, peer::{Peer, RemotePeer}
};

struct MyObserver {
   name: String,
}

#[async_trait]
impl Observer for MyObserver {
    async fn on_connected(&mut self, remote: &RemotePeer) -> Option<Response> {
        let mut text = String::from("Hello I am ");
        text.push_str(&self.name);
        Some(Response::text(&text))
    }

    async fn on_disconnected(&mut self, remote: &RemotePeer) -> Option<Response> {
        Some(Response::text("Goodbye !"))
    }

    async fn on_message(&mut self, m: &Message) -> Option<Response> {
        println!("{} : {}", self.name, String::from_utf8(m.content.clone()).unwrap());
        None
    }
}

#[cfg(not(feature = "ssl"))]
async fn example() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let observer1 = MyObserver{
        name: String::from("Peer1"),
    };
    let peer1 = Peer::new(
        Configuration::builder().port(9001).build(),
        InMemoryDao::default(),
        observer1,
    ).await;

    let observer2 = MyObserver{
        name: String::from("Peer2"),
    };
    let peer2 = Peer::new(
        Configuration::builder().port(9002).build(),
        InMemoryDao::default(),
        observer2,
    ).await;

    peer1.connect_to(&peer2.addr());

    peer1.close();
    peer2.close();
}

#[cfg(feature = "ssl")]
async fn example() {
    use rudp2plib::configuration::SSL;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let observer1 = MyObserver{
        name: String::from("Peer1"),
    };
    let peer1 = Peer::new(
        Configuration::builder(SSL::default()).port(9001).build(),
        InMemoryDao::default(),
        observer1,
    ).await;

    let observer2 = MyObserver{
        name: String::from("Peer2"),
    };
    let peer2 = Peer::new(
        Configuration::builder(SSL::default()).port(9002).build(),
        InMemoryDao::default(),
        observer2,
    ).await;

    peer1.connect_to(&peer2.addr());

    peer1.close();
    peer2.close();
}

fn main() {
    futures::executor::block_on(example());
}
```

## Features
- [default](./reports/default.md)
- [ssl](./reports/ssl.md)
