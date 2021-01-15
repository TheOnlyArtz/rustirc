use super::message_parser::{Message, Command};
use super::client::{ClientState, Client};
use super::event_handler::EventHandler;
use std::sync::Arc;

pub async fn handle_event(client: &mut Client, message: Message, event_handler: &Arc<dyn EventHandler>) -> Result<(), Box<dyn std::error::Error>>{
    println!("{:#?} => {:?}", message.command, message.parameters);
    match message.command {
        Command::Notice => {
            match client.state {
                ClientState::Connecting => {
                    // Start the registering process
                    client.state = ClientState::Registering;
                    client.register("RustIRCBot").await?;
                },
                _ => {}
            }
        },
        Command::RplWelcome => {
            // dispatch a welcome event log for now
            println!("Client has connected successfully to the server");
            event_handler.on_server_connect(client, message).await;
            
        },
        Command::PrivMsg => {
            event_handler.on_message_sent(client, message).await;
        }
        _ => {}
    }

    Ok(())
}