use crate::ServerMessage;
use std::sync::{mpsc::Sender, Arc};

#[derive(Clone)]
pub struct ServerMessageSender {
    sender: Arc<Sender<ServerMessage>>,
}

impl ServerMessageSender {
    pub fn new(sender: Sender<ServerMessage>) -> Self {
        Self {
            sender: Arc::new(sender),
        }
    }

    pub fn send(&self, server_message: ServerMessage) {
        let _ = self.sender.send(server_message); // todo check result
    }
}
