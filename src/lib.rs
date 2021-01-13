use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;
use tokio::io::BufReader;
use tokio::net::TcpStream;
use tokio::prelude::*;

// use tokio::net::tcp::{ReadHalf, WriteHalf};
pub struct Client {
    ip: String,
    port: u16,
    event_handler: Option<Arc<dyn EventHandler>>,
    pub stream: Option<IrcStream<TcpStream>>
}

pub enum ClientState {
    Connecting,
    Registering,
    InChannel
}

impl Client {
    pub fn new(ip: &str, port: u16) -> Client {
        Client {
            ip: ip.to_string(),
            port,
            event_handler: None,
            stream: None
        }
    }

    pub fn handler<H: EventHandler + 'static>(mut self, event_handler: H) -> Self {
        self.event_handler = Some(Arc::new(event_handler));
        self
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        self.stream = Some(IrcStream::connect(&self.ip, self.port).await?);
        Ok(())
    }
}

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn test(&self) {}
}

async fn dispatch(event_handler: &Option<Arc<dyn EventHandler>>) {
    match event_handler {
        Some(eh) => eh.test().await,
        None => {}
    }
}

pub struct IrcStream<S> {
    reader: BufReader<S>
}

impl IrcStream<TcpStream> {
    pub async fn connect(peer: &str, port: u16) -> Result<Self, Box<dyn Error>> {
        let connection = TcpStream::connect(&format!("{}:{}", peer, port)).await?;
        Ok(IrcStream::new(connection))
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> IrcStream<S> {
    pub fn new(stream: S) -> Self {
        IrcStream { reader: BufReader::new(stream) }
    }

    pub async fn consume_message(&mut self) -> Result<(Vec<u8>, usize), Box<dyn Error>> {
        let mut buf = Vec::new();
        let test = self.reader.read_until(b'\n', &mut buf).await?;
        
        Ok((buf, test))
    }

    pub async fn write_all(&mut self, test: &[u8]) -> Result<(), Box<dyn Error>> {
        self.reader.write(test).await?;
        Ok(())
    }
}