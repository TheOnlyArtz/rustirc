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
                    client.register("RustIRCBot").await?; // TODO: make it dynamic veia a Client field
                }
                _ => {}
            }
        }
        Command::RplWelcome => {
            // dispatch a welcome event log for now
            client.state = ClientState::InServer;
            event_handler.on_server_connect(client, message).await;
        }
        Command::RplMotdStart | Command::RplMotd => {
            event_handler.on_message_of_the_day(client, message).await;
        }
        Command::PrivMsg => {
            event_handler.on_message_sent(client, message).await;
        },
        Command::Join => {
            client.state = ClientState::InChannel((&message.parameters[0][1..]).to_owned());
            event_handler.on_channel_join(client, message).await;
        }
        Command::Ping => {
            client.send_pong().await?;
        },
        _ => {
            println!("{:#?} => {:?}", message.command, message.parameters);
        }
    }

    Ok(())
}
