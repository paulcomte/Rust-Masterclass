use crate::io;
use std::net::SocketAddr;
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::sync::{mpsc, Arc};
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWrite, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

#[derive(PartialEq)]
pub enum TransporterStatus {
    Active,
    Stopped,
}

#[derive(Clone)]
pub struct Transporter {
    pub receiver: Arc<Mutex<Receiver<io::Message>>>,
    pub writer: Arc<Mutex<Box<dyn AsyncWrite + Send + Unpin + 'static>>>,
    pub writes: Arc<Mutex<usize>>,
    pub status: Arc<Mutex<TransporterStatus>>,
    pub peer_addr: SocketAddr,
}

impl Transporter {
    pub async fn new(stream: TcpStream) -> (Self, JoinHandle<()>) {
        let peer_addr = stream.peer_addr().unwrap();
        let (reader, writer) = stream.into_split();
        let (receiver, handle) = Self::spawn_receiving_channel(Box::new(reader)).await;

        (
            Self {
                receiver: Arc::new(Mutex::new(receiver)),
                writer: Arc::new(Mutex::new(Box::new(writer))),
                writes: Arc::new(Mutex::new(0)),
                status: Arc::new(Mutex::new(TransporterStatus::Active)),
                peer_addr,
            },
            handle,
        )
    }

    async fn spawn_receiving_channel(
        mut reader: Box<impl AsyncReadExt + Unpin + Send + 'static>,
    ) -> (Receiver<io::Message>, JoinHandle<()>) {
        let (tx, rx) = mpsc::channel();
        let handle = tokio::spawn(async move {
            loop {
                let mut response = String::new();
                let mut stream = BufReader::new(&mut reader);

                match tx.send(io::Message::parse_next_line(
                    stream.read_line(&mut response).await,
                    response.into(),
                )) {
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
        });
        (rx, handle)
    }

    pub async fn send_bytes(&self, bytes: &[u8]) {
        self.new_write().await;
        self.writer.lock().await.write_all(bytes).await.unwrap();
        self.end_write().await;
    }

    pub async fn send_message<S: Into<String>>(&self, message: S) {
        let message = message.into();
        self.send_bytes(message.as_bytes()).await;
    }

    pub async fn send_message_with_response<S: Into<String>>(
        &self,
        message: S,
        timeout: Duration,
    ) -> Result<io::Message, RecvTimeoutError> {
        let receiver = self.receiver.lock().await;
        self.send_message(message).await;
        receiver.recv_timeout(timeout)
    }

    async fn new_write(&self) {
        *self.writes.lock().await += 1;
    }

    async fn end_write(&self) {
        *self.writes.lock().await -= 1;
    }

    pub async fn shutdown(&self) {
        self.writer.lock().await.shutdown().await.unwrap();
    }

    pub async fn stop(&self) {
        *self.status.lock().await = TransporterStatus::Stopped;
        self.shutdown().await;
    }

    pub async fn is_active(&self) -> bool {
        *self.status.lock().await == TransporterStatus::Active
    }
}
