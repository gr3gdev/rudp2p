use std::fs::File;

use cucumber::{gherkin::Step, given, then, when, World, writer};

use crate::common::{log, PeersWorld};
use crate::report::report_cucumber;

mod common;
mod report;

#[given(expr = "the following peers are started")]
async fn start_peers(w: &mut PeersWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) { // NOTE: skip header
            let name = &row[0];
            let port = row[1].parse::<u16>().unwrap();
            w.add_peer(name.clone(), port, vec![], None);
        }
    }
}

#[when(expr = "the following peers connect to {string}")]
async fn connect_peer(w: &mut PeersWorld, dispatcher_name: String, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) { // NOTE: skip header
            let peer_name = &row[0];
            let port = row[1].parse::<u16>().unwrap();
            let mut authorized_peers = Vec::new();
            if row.len() == 3 {
                let peers = &row[2].split(",").collect::<Vec<&str>>();
                for peer in peers {
                    authorized_peers.push(peer.to_string());
                }
                log(format!("{} {:?}", peer_name, authorized_peers));
            }
            w.add_peer(peer_name.clone(), port, authorized_peers, Some(dispatcher_name.clone()));
        }
    }
}

#[when(expr = "the peer {string} disconnects")]
async fn disconnect_peer(w: &mut PeersWorld, peer_name: String) {
    w.remove_peer(peer_name);
}

#[when(expr = "the peer {string} sends {string} to all")]
async fn peer_sends_to_all(w: &mut PeersWorld, peer_name: String, data: String) {
    let peer_data = w.find(&peer_name);
    peer_data.send_to_all(data);
}

#[when(expr = "the peer {string} sends {string} to {string}")]
async fn peer_sends_to_peer(w: &mut PeersWorld, peer_name: String, data: String, other_peer: String) {
    let peer_data = w.find(&peer_name);
    peer_data.send_to(data, &other_peer);
}

#[then(expr = "the peer {string} does not receives")]
async fn not_receive_event(w: &mut PeersWorld, peer_name: String, step: &Step) {
    let peer_data = w.find(&peer_name);
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) { // NOTE: skip header
            let event = &row[0];
            let sender_name = &row[1];
            if event == "Connected" {
                peer_data.is_not_connected_with(sender_name);
            } else if event == "Disconnected" {
                peer_data.is_not_disconnected_with(sender_name);
            } else {
                peer_data.is_not_message_received(sender_name);
            }
        }
    }
}

#[then(expr = "the peer {string} receives")]
async fn receive_event(w: &mut PeersWorld, peer_name: String, step: &Step) {
    let peer_data = w.find(&peer_name);
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) { // NOTE: skip header
            let event = &row[0];
            let sender_name = &row[1];
            if event == "Connected" {
                peer_data.is_connected_with(sender_name);
            } else if event == "Disconnected" {
                peer_data.is_disconnected_with(sender_name);
            } else {
                peer_data.is_message_received(event.clone(), sender_name);
            }
        }
    } else {
        panic!("Peer not exist with the name : {}", peer_name);
    }
}

#[when(expr = "the peer {string} blocks the peer {string}")]
async fn block_peer(w: &mut PeersWorld, peer_name: String, blocked_peer_name: String) {
    let peer_data = w.find(&peer_name);
    peer_data.peer.block_peers(vec![blocked_peer_name]);
}

fn main() {
    let file = File::create("target/cucumber.json").expect("Unable to create cucumber.json");
    let writer = writer::Json::new(file);
    futures::executor::block_on(PeersWorld::cucumber()
        .with_writer(writer)
        .after(|_feature, _rule, _scenario, _ev, world| {
            world.unwrap().close()
        })
        .run("features"));
    report_cucumber("target/cucumber.json");
}
