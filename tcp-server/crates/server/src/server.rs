//! --- Server Module ---
//!
//! A TCP server implementation

use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::server_message::{ServerMessageHandler, ServerMessageSender};

use crate::{tcp, PacketHandler, ServerManager};

pub struct Server {
    bind_hostname: String,
    bind_port: u16,
    is_on: bool,
    server_manager: Arc<Mutex<ServerManager>>,
}

impl Server {
    pub fn new(
        bind_hostname: String,
        bind_port: u16,
        server_manager: Arc<Mutex<ServerManager>>,
    ) -> Self {
        Self {
            bind_hostname: bind_hostname.into(),
            bind_port,
            is_on: true,
            server_manager,
        }
    }

    async fn socket_handler(&mut self, sender: ServerMessageSender, listener: &TcpListener) {
        let socket = listener.accept().await;
        let acceptor = tcp::Acceptor::new(self.server_manager.clone(), sender);
        tokio::spawn(async move {
            acceptor.handle_incoming(socket).await;
        });
    }

    pub async fn run(&mut self, packet_handler: impl PacketHandler + 'static) {
        let bind_addr = self.bind_addr();
        let (server_message_handler, sender) =
            ServerMessageHandler::new(self.server_manager.clone());

        let handle = server_message_handler.listen(packet_handler).await;

        let listener = TcpListener::bind(&bind_addr).await.unwrap();
        eprintln!("Server is now listening on [{:?}]", &bind_addr);
        while self.is_on {
            self.socket_handler(sender.clone(), &listener).await;
        }

        handle.await.expect("Could not join");
    }

    pub fn bind_addr(&self) -> String {
        format!(
            "{bind_hostname}:{bind_port}",
            bind_hostname = self.bind_hostname,
            bind_port = self.bind_port
        )
    }
}
