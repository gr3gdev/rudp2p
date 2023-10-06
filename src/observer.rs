use crate::network::{events::Message, Response};
use crate::peer::RemotePeer;
use async_trait::async_trait;

/// # Observer
/// 
/// Trait for receive events.
#[async_trait]
pub trait Observer: Send + 'static {
    /// A another Peer is connected.
    async fn on_connected(&mut self, remote: &RemotePeer) -> Option<Response>;
    /// A another Peer is disconnected.
    async fn on_disconnected(&mut self, remote: &RemotePeer) -> Option<Response>;
    /// A another Peer send a message.
    async fn on_message(&mut self, m: &Message) -> Option<Response>;
}
