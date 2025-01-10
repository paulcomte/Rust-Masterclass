use std::sync::{
    mpsc::{channel, Receiver, TryRecvError},
    Arc,
};

use tokio::{sync::Mutex, task::JoinHandle};

use crate::{
    server_message::{ServerMessage, ServerMessageSender},
    PacketHandler, ServerManager,
};

pub struct ServerMessageHandler {
    receiver: Receiver<ServerMessage>,
    server_manager: Arc<Mutex<ServerManager>>,
}

impl ServerMessageHandler {
    pub fn new(server_manager: Arc<Mutex<ServerManager>>) -> (Self, ServerMessageSender) {
        let (sender, receiver) = channel();
        (
            Self {
                receiver,
                server_manager,
            },
            ServerMessageSender::new(sender),
        )
    }

    pub async fn listen(self, packet_handler: impl PacketHandler + 'static) -> JoinHandle<()> {
        tokio::spawn(async move {
            self._listen(packet_handler).await;
        })
    }

    async fn _listen(mut self, mut packet_handler: impl PacketHandler) {
        loop {
            match self.receiver.try_recv() {
                Ok(message) => {
                    self.handle_message(message, &mut packet_handler).await;
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    eprintln!("Channel disconnected");
                    break;
                }
            }
        }
    }

    async fn handle_message(
        &mut self,
        message: ServerMessage,
        packet_handler: &mut impl PacketHandler,
    ) {
        match message {
            ServerMessage::HandlePacket(io_client, packet) => {
                let packet = packet::deserialize(&packet);

                match packet {
                    Ok(packet) => {
                        packet_handler
                            .handle(self.server_manager.clone(), io_client, packet)
                            .await
                    }
                    Err(err) => {
                        eprintln!("Error whilst deserializing packet: {err:#?}");
                    }
                }
            }
            ServerMessage::Disconnect(client) => {
                self.server_manager.lock().await.remove_client(&client);
            }
        }
    }
}
