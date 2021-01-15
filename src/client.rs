use super::event_handler::EventHandler;
use super::event_manager;
use super::message_parser;
use std::error::Error;
use std::sync::Arc;
use tokio::io::BufReader;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::sync::Mutex;
// use tokio::net::tcp::{ReadHalf, WriteHalf};

pub enum ClientState {
    Uninit,
    Connecting,
    Registering,
    InChannel,
}

pub struct Client {
    ip: String,
    port: u16,
    event_handler: Option<Arc<dyn EventHandler>>,
    pub state: ClientState,
    pub stream: Option<Arc<Mutex<IrcStream<TcpStream>>>>,
}

impl Client {
    pub fn new(ip: &str, port: u16) -> Client {
        Client {
            ip: ip.to_string(),
            port,
            event_handler: None,
            state: ClientState::Uninit,
            stream: None,
        }
    }

    pub fn handler<H: EventHandler + 'static>(mut self, event_handler: H) -> Self {
        self.event_handler = Some(Arc::new(event_handler));
        self
    }

    pub async fn connect(mut self) -> Result<Self, Box<dyn Error>> {
        self.stream = Some(Arc::new(Mutex::new(
            IrcStream::connect(&self.ip, self.port).await?,
        )));
        self.state = ClientState::Connecting;
        Ok(self)
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let event_handler = self.event_handler.take().unwrap();

        loop {
            let stream = Arc::clone(&self.stream.as_ref().unwrap());
            let mut s = stream.try_lock().unwrap();
            let message = s.consume_message().await.unwrap();
            std::mem::drop(s); // Drop the lock on the stream

            let message = message_parser::parse_message(&message.0).unwrap();
            if let Err(e) = event_manager::handle_event(self, message, &event_handler).await {
                eprintln!("Error: {}", e);
                break
            }

            // if message.command.len() == 0 {
            //     break;
            // }

            // match command {
            //     "NOTICE" => {
            //         println!("{:?}", message);
            //         if !sent_reg {
            //             // self.register(username).await?;

            //             // println!("Sent register");
            //         }
            //         sent_reg = true;
            //     }
            //     "376" => {
            //         self.join_channel("channel").await?;
            //     }
            //     "PRIVMSG" => {
            //         event_handler.on_message_sent(self, message).await;
            //     }
            //     "PING" => {
            //         self.send_pong().await?;
            //         println!("SENT {:?}", message);
            //     }
            //     _ => {
            //         println!("{:?}", message);
            //     }
            // }
        }

        Ok(())
    }

    pub async fn register(&mut self, username: &str) -> Result<(), Box<dyn Error>> {
        send_socket_message(self, &format!("PASS {}", username)).await?;
        send_socket_message(self, &format!("NICK {}", username)).await?;
        send_socket_message(self, &format!("USER guest * 0 :{}", username)).await?;
        Ok(())
    }
    pub async fn join_channel(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        send_socket_message(self, &format!("JOIN #{}", name)).await?;
        Ok(())
    }

    pub async fn send_message(&mut self, msg: String) -> Result<(), Box<dyn Error>> {
        send_socket_message(self, &format!("PRIVMSG #channel {}", msg)).await?;
        Ok(())
    }

    pub async fn send_pong(&mut self) -> Result<(), Box<dyn Error>> {
        send_socket_message(self, "PONG").await?;
        Ok(())
    }
}

pub struct IrcStream<S> {
    reader: BufReader<S>,
}

impl IrcStream<TcpStream> {
    pub async fn connect(peer: &str, port: u16) -> Result<Self, Box<dyn Error>> {
        let connection = TcpStream::connect(&format!("{}:{}", peer, port)).await?;
        Ok(IrcStream::new(connection))
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> IrcStream<S> {
    pub fn new(stream: S) -> Self {
        IrcStream {
            reader: BufReader::new(stream),
        }
    }

    pub async fn consume_message(&mut self) -> Result<(Vec<u8>, usize), Box<dyn Error>> {
        let mut buf = Vec::new();
        let test = self.reader.read_until(b'\n', &mut buf).await?;
        Ok((buf, test))
    }

    pub async fn write_all(&mut self, test: &str) -> Result<(), Box<dyn Error>> {
        self.reader
            .write(&format!("{}\r\n", test)[..].as_bytes())
            .await?;
        Ok(())
    }
}

pub async fn send_socket_message(client: &mut Client, msg: &str) -> Result<(), Box<dyn Error>> {
    let stream = Arc::clone(client.stream.as_ref().unwrap());
    let mut s = stream.try_lock().unwrap();

    s.write_all(msg).await?;
    Ok(())
}
