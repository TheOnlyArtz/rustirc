use async_trait::async_trait;
use super::client::Client;
use super::message_parser::Message;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn on_message_sent(&self, client: &mut Client, params: Message) {}
}