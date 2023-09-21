use async_trait::async_trait;

use crate::network::{
    events::{Connected, Disconnected, Message},
    Response,
};

#[async_trait]
pub trait Observer: Send + 'static {
    async fn on_connected(&mut self, c: Connected) -> Option<Response>;
    async fn on_disconnected(&mut self, d: Disconnected) -> Option<Response>;
    async fn on_message(&mut self, m: Message) -> Option<Response>;
}
