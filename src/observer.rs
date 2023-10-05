use crate::network::{events::Message, Response};
use crate::peer::RemotePeer;
use async_trait::async_trait;

#[async_trait]
pub trait Observer: Send + 'static {
    async fn on_connected(&mut self, remote: &RemotePeer) -> Option<Response>;
    async fn on_disconnected(&mut self, remote: &RemotePeer) -> Option<Response>;
    async fn on_message(&mut self, m: &Message) -> Option<Response>;
}
