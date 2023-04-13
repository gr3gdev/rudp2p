use crate::peer::event::common::Parser;
use crate::peer::event::message::PeerMessageEvent;
use crate::peer::event::PeerEvent;
use crate::peer::message::PeerMessage;
use crate::peer::router::Router;

// STRUCT

pub(crate) struct MessageRouter;

// IMPL

impl MessageRouter {
    pub(crate) fn execute(peer_event: PeerEvent, router: &Router) {
        let guard_message = router.shared_message.lock().unwrap();
        if let Some(ref mut observer) = *guard_message.borrow_mut() {
            let peer_message = PeerMessageEvent::parse(&peer_event.message);
            observer(&PeerMessage::parse(&peer_message.content), &peer_message.uid);
        };
    }
}