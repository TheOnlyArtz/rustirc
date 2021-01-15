use async_trait::async_trait;
use super::client::Client;
use super::message_parser::Message;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn on_message_sent(&self, client: &mut Client, params: Message) {}
    async fn on_server_connect(&self, client: &mut Client, params: Message) {}
    async fn on_channel_join(&self, client: &mut Client, params: Message) {}
    async fn on_message_of_the_day(&self, client: &mut Client, params: Message) {}
    async fn on_unimplemented(&self, client: &mut Client, params: Message) {}
}