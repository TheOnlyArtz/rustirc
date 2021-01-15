use super::client::{Client, ClientState};
use super::event_handler::EventHandler;
use super::message_parser::{Command, Message};
use std::sync::Arc;

pub async fn handle_event(
    client: &mut Client,
    message: Message,
    event_handler: &Arc<dyn EventHandler>,
) -> Result<(), Box<dyn std::error::Error>> {
    match message.command {
        Command::Notice => {
            match client.state {
                ClientState::Connecting => {
                    // Start the registering process
                    client.state = ClientState::Registering;
                    client.register("RustIRCBot").await?;
                }
                _ => {}
            }
        }
        Command::RplWelcome => {
            // dispatch a welcome event log for now
            println!("Client has connected successfully to the server");
            client.state = ClientState::InServer;
            event_handler.on_server_connect(client, message).await;
        }
        Command::RplMotdStart | Command::RplMotd => {
            event_handler.on_message_of_the_day(client, message).await;
        }
        Command::PrivMsg => {
            event_handler.on_message_sent(client, message).await;
        },
        Command::Ping => {
            println!("Sending PING");
            client.send_pong().await?;
        },
        _ => {
            println!("{:#?} => {:?}", message.command, message.parameters);
        }
    }

    Ok(())
}
