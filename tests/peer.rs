use std::fs;

use crate::data::{DataTable, Event, PeerData, TextMessage};
use crate::utils::log;
use cucumber::{gherkin::Step, given, then, when, World};
use data::PeersWorld;

pub(crate) mod data;
pub(crate) mod utils;

#[given(expr = "the following peers are started")]
async fn start_peers(world: &mut PeersWorld, step: &Step) {
    let peers = PeerData::read_from_datatable(step);
    world.add_all(peers);
}

#[when(expr = "the peer {string} connects to {string}")]
async fn connect_peer(world: &mut PeersWorld, from: String, to: String) {
    log(format!("{} connects to {}", from, to));
    let peer1 = world.get_peer(from);
    let peer2 = world.get_peer(to);
    let peer2_addr = peer2.addr().expect("Unable to get peer address");
    assert!(peer1.connect(peer2_addr.clone()).is_ok());
}

#[when(expr = "the peer {string} disconnects")]
async fn disconnect_peer(world: &mut PeersWorld, peer_name: String) {
    log(format!("{} disconnects", peer_name));
    let peer = world.get_peer(peer_name);
    assert!(peer.disconnect_to_all().is_ok());
}

#[when(expr = "the peer {string} sends {string} to all")]
async fn peer_sends_to_all(world: &mut PeersWorld, peer_name: String, data: String) {
    log(format!("{} sends {} to all", peer_name, data));
    let peer = world.get_peer(peer_name);
    assert!(peer.send_all(TextMessage { text: data }).is_ok());
}

#[when(expr = "the peer {string} sends {string} to {string}")]
async fn peer_sends_to_peer(
    world: &mut PeersWorld,
    peer_name: String,
    data: String,
    other_peer: String,
) {
    log(format!("{} sends {} to {}", peer_name, data, other_peer));
    let peer = world.get_peer(peer_name);
    assert!(peer.send_to(TextMessage { text: data }, other_peer).is_ok());
}

#[when(expr = "the peer {string} blocks the peer {string}")]
async fn block_peer(world: &mut PeersWorld, peer_name: String, blocked_peer_name: String) {
    let peer = world.get_peer(peer_name);
    peer.block_peers(vec![blocked_peer_name]);
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
    fs::remove_file("target/tests.log").unwrap_or_else(|_| println!("tests.log does not exists"));
    futures::executor::block_on(
        PeersWorld::cucumber()
            .after(|_feature, _rule, _scenario, _ev, world| world.unwrap().close())
            .run("features"),
    );
}
