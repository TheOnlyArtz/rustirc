# rustirc
A fully asynchronous IRC client written in Rust

### A fully working echo bot example
```rust
use rustirc::{
    client::{Client, ClientState},
    event_handler::EventHandler,
    message_parser,
};

struct Handler;

#[async_trait::async_trait]
impl EventHandler for Handler {
    async fn on_message_sent(&self, client: &mut Client, message: message_parser::Message) {
        println!(
            "Message content => {} | sent by: {}",
            message.parameters[1],
            message.source.unwrap()
        );

        // That means we're in a channel and we can send a message
        match client.state {
            ClientState::InChannel(_) => {
                client
                    .send_message(message.parameters[1])
                    .await;
            }
            _ => {}
        }
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut c = Client::new("localhost", 6667)
        .handler(Handler)
        .connect()
        .await?;

    c.start().await?;
    Ok(())
}
```
