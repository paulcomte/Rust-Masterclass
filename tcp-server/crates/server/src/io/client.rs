use packet::Packet;

use super::Message;
use crate::server_message::{ServerMessage, ServerMessageSender};
use crate::tcp::Transporter;
use std::sync::mpsc::RecvTimeoutError;
use std::time::Duration;

#[derive(Clone)]
pub struct ClientId(pub usize);

#[derive(Clone)]
pub struct Client {
    pub id: ClientId,
    transporter: Transporter,
    server_message_sender: ServerMessageSender,
}

impl Client {
    pub async fn new(
        id: ClientId,
        transporter: Transporter,
        server_message_sender: ServerMessageSender,
    ) -> Self {
        Self {
            id,
            transporter,
            server_message_sender,
        }
    }

    pub async fn stop(&self) {
        self.transporter.stop().await;
    }

    pub async fn welcome(&self) {
        let ping_packet = packet::ping::create(packet::ping::PingType::Ping);
        let encoded = packet::ping::serialize(&ping_packet);
        let packet = packet::create(packet::packet::PacketType::Ping, encoded);
        self.send_packet(packet).await;
    }

    pub async fn handle_message(&self, message: Message) -> bool {
        match message.content {
            Ok(Some(message)) => {
                println!("[{}]:[{:?}]", message.len(), message);
                let server_message = ServerMessage::HandlePacket(self.clone(), message);
                self.server_message_sender.send(server_message);
                true
            }
            Ok(None) => false,
            Err(err) => {
                eprintln!("err = [{:?}]", err);
                eprintln!(
                    "An error occurred, terminating connection with {}",
                    self.transporter.peer_addr
                );
                self.transporter.shutdown().await;
                false
            }
        }
    }

    pub async fn send_packet(&self, packet: Packet) {
        let mut bytes = packet::serialize(&packet);
        bytes.extend_from_slice(b"\r\n");
        let transporter = self.transporter.clone();

        let _ = tokio::spawn(async move {
            transporter.send_bytes(&bytes).await;
        })
        .await;
    }

    pub async fn send_message<S: Into<String>>(&self, message: S) {
        let message = message.into();
        let transporter = self.transporter.clone();

        let _ = tokio::spawn(async move {
            transporter.send_message(message).await;
        })
        .await;
    }

    /// Send a message and returns its response
    pub async fn send_message_with_response<S: Into<String>>(
        &self,
        message: S,
        timeout: Duration,
    ) -> Result<Message, RecvTimeoutError> {
        self.transporter
            .send_message_with_response(format!("{message}\r\n", message = message.into()), timeout)
            .await
    }

    pub async fn listen(self) {
        tokio::spawn(async move {
            self._listen().await;
            if self.transporter.is_active().await {
                while *self.transporter.writes.lock().await != 0 {
                    eprintln!("waiting for shutdown call");
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }
                self.transporter.shutdown().await;
            }
            self.server_message_sender
                .send(ServerMessage::Disconnect(self.clone()));
        })
        .await
        .expect("COULD NOT START LISTENING");
    }

    async fn _listen(&self) {
        loop {
            let receiver = self.transporter.receiver.clone();
            match receiver.lock().await.recv() {
                Ok(message) => {
                    if !self.handle_message(message).await {
                        println!("can't handle the message, breaking");
                        break;
                    }
                }
                Err(_) => {}
            }
            if !self.transporter.is_active().await {
                println!("breaking");
                break;
            }
        }
    }
}
