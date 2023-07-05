use crate::data::{DataTable, Event, PeerData};
use crate::utils::read_file;
use cucumber::{gherkin::Step, given, then, when, World};
use data::PeersWorld;
use rudp2plib::Message;

pub(crate) mod data;
pub(crate) mod utils;

#[given(expr = "the following peers are started")]
async fn start_peers(world: &mut PeersWorld, step: &Step) {
    let peers = PeerData::read_from_datatable(step);
    world.add_all(peers);
}

#[when(expr = "the peer {string} connects to {string}")]
async fn connect_peer(world: &mut PeersWorld, from: String, to: String) {
    let peer1 = world.get_peer(from);
    let peer2 = world.get_peer(to);
    let peer2_addr = peer2.addr();
    peer1.connect_to(peer2_addr.clone());
}

#[when(expr = "the peer {string} disconnects")]
async fn disconnect_peer(world: &mut PeersWorld, peer_name: String) {
    let peer = world.get_peer(peer_name);
    peer.disconnect_to_all();
}

#[when(expr = "the peer {string} sends {string} to all")]
async fn peer_sends_to_all(world: &mut PeersWorld, peer_name: String, data: String) {
    let peer = world.get_peer(peer_name);
    peer.send_to_all(Message::new(data.as_bytes().to_vec()));
}

#[when(expr = "the peer {string} sends {string} to {string}")]
async fn peer_sends_to_peer(
    world: &mut PeersWorld,
    peer_name: String,
    data: String,
    other_peer: String,
) {
    let peer = world.get_peer(peer_name);
    if data.starts_with("file:") {
        let data_file = read_file(&data[5..]);
        assert!(peer.send_to(Message::new(data_file), other_peer).is_ok());
    } else {
        assert!(peer
            .send_to(Message::new(data.as_bytes().to_vec()), other_peer)
            .is_ok());
    }
}

#[when(expr = "the peer {string} blocks the peer {string}")]
async fn block_peer(world: &mut PeersWorld, peer_name: String, blocked_peer_name: String) {
    let peer = world.get_peer(peer_name);
    peer.block(blocked_peer_name);
}

#[then(expr = "the peer {string} does not receives")]
async fn not_receive_event(world: &mut PeersWorld, peer_name: String, step: &Step) {
    let events = Event::read_from_datatable(step);
    world.check_peer_not_receive(peer_name, events);
}

#[then(expr = "the peer {string} receives")]
async fn receive_event(world: &mut PeersWorld, peer_name: String, step: &Step) {
    let events = Event::read_from_datatable(step);
    world.check_peer_receive(peer_name, events);
}

fn main() {
    futures::executor::block_on(
        PeersWorld::cucumber()
            .after(|_feature, _rule, _scenario, _ev, world| world.unwrap().close())
            .run("features"),
    );
}
