use std::collections::HashMap;

use crate::data::{DataTable, Event, PeerData};
use crate::utils::read_file;
use cucumber::{gherkin::Step, given, then, when, World};
use data::PeersWorld;
use rudp2plib::network::Request;

pub(crate) mod dao;
pub(crate) mod data;
pub(crate) mod utils;

#[given(expr = "the following peers are started")]
async fn start_peers(world: &mut PeersWorld, step: &Step) {
    let peers = PeerData::read_from_datatable(step);
    world.add_all(peers).await;
}

#[when(expr = "the peer {string} connects to {string}")]
async fn connect_peer(world: &mut PeersWorld, from: String, to: String) {
    let peer1 = world.get_peer(&from);
    let peer2 = world.get_peer(&to);
    let peer2_addr = peer2.addr();
    peer1.connect_to(&peer2_addr);
}

#[when(expr = "the peer {string} disconnects")]
async fn disconnect_peer(world: &mut PeersWorld, peer_name: String) {
    let peer = world.get_peer(&peer_name);
    peer.disconnect_to_all().await;
}

#[when(expr = "the peer {string} sends {string} to all")]
async fn peer_sends_to_all(world: &mut PeersWorld, peer_name: String, data: String) {
    let peer = world.get_peer(&peer_name);
    peer.send_to_all(&Request::new(data)).await;
}

#[when(expr = "the peer {string} sends {string} to {string}")]
async fn peer_sends_to_peer(
    world: &mut PeersWorld,
    peer_name: String,
    data: String,
    other_peer: String,
) {
    let peer = world.get_peer(&peer_name);
    let other_peer = world.get_peer(&other_peer);
    if data.starts_with("file:") {
        let data_file = read_file(&data[5..]);
        peer.send_to(Request::new(data_file), &other_peer.addr())
            .await;
    } else {
        peer.send_to(Request::new(data), &other_peer.addr()).await;
    }
}

#[when(expr = "the peer {string} blocks the peer {string}")]
async fn block_peer(world: &mut PeersWorld, peer_name: String, blocked_peer_name: String) {
    let peer = world.get_peer(&peer_name);
    let blocked_peer = world.get_peer(&blocked_peer_name);
    peer.block(&blocked_peer.addr()).await;
}

#[when(expr = "the peer {string} unblocks the peer {string}")]
async fn unblock_peer(world: &mut PeersWorld, peer_name: String, blocked_peer_name: String) {
    let peer = world.get_peer(&peer_name);
    let blocked_peer = world.get_peer(&blocked_peer_name);
    peer.unblock(&blocked_peer.addr()).await;
}

fn group_event_by_type(events: Vec<Event>) -> HashMap<String, Vec<Event>> {
    let mut events_by_type = HashMap::new();
    for e in events {
        let event_type = e.event.clone();
        match events_by_type.entry(event_type.clone()) {
            std::collections::hash_map::Entry::Occupied(mut o) => {
                let list: &mut Vec<Event> = o.get_mut();
                list.push(e);
            }
            std::collections::hash_map::Entry::Vacant(_) => {
                events_by_type.insert(event_type, vec![e.clone()]);
            }
        }
    }
    events_by_type
}

#[then(expr = "the peer {string} does not receives")]
async fn not_receive_event(world: &mut PeersWorld, peer_name: String, step: &Step) {
    let events = Event::read_from_datatable(step);
    let events_by_type = group_event_by_type(events);
    for (event_type, events) in events_by_type {
        if !events.is_empty() {
            world
                .check_peer_not_receive(peer_name.clone(), events, event_type)
                .await;
        }
    }
}

#[then(expr = "the peer {string} receives")]
async fn receive_event(world: &mut PeersWorld, peer_name: String, step: &Step) {
    let events = Event::read_from_datatable(step);
    let events_by_type = group_event_by_type(events);
    for (event_type, events) in events_by_type {
        if !events.is_empty() {
            world
                .check_peer_receive(peer_name.clone(), events, event_type)
                .await;
        }
    }
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .format_indent(Some(2))
        .format_module_path(false)
        .format_target(true)
        .init();
    futures::executor::block_on(
        PeersWorld::cucumber()
            .after(|_feature, _rule, _scenario, _ev, world| world.unwrap().close())
            .run("features"),
    );
}
