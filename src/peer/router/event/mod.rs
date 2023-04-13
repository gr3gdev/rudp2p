use std::net::{SocketAddr, UdpSocket};

use crate::peer::event::connected::router::ConnectedRouter;
use crate::peer::event::connecting::router::ConnectingRouter;
use crate::peer::event::ident::router::{DisconnectedRouter, DisconnectingRouter};
use crate::peer::event::message::router::MessageRouter;
use crate::peer::event::PeerEvent;
use crate::peer::router::Router;

pub(crate) trait Route {
    fn execute(&self, peer_event: PeerEvent, socket: &UdpSocket, sender: SocketAddr, router: &Router);
}

pub(crate) enum RouterEvent {
    Disconnecting,
    Connecting,
    Connected,
    Message,
    Disconnected,
}

impl RouterEvent {
    pub(crate) fn find_by_code(code: u8) -> RouterEvent {
        match code {
            0 => RouterEvent::Disconnecting,
            1 => RouterEvent::Connecting,
            2 => RouterEvent::Connected,
            3 => RouterEvent::Message,
            4 => RouterEvent::Disconnected,
            _ => panic!("Event not found !")
        }
    }
}

impl Route for RouterEvent {
    fn execute(&self, peer_event: PeerEvent, socket: &UdpSocket, sender: SocketAddr, router: &Router) {
        match self {
            RouterEvent::Disconnecting => DisconnectingRouter::execute(peer_event, socket, sender, router),
            RouterEvent::Connecting => ConnectingRouter::execute(peer_event, socket, sender, router),
            RouterEvent::Connected => ConnectedRouter::execute(peer_event, socket, router),
            RouterEvent::Message => MessageRouter::execute(peer_event, router),
            RouterEvent::Disconnected => DisconnectedRouter::execute(peer_event, router),
        }
    }
}
