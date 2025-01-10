use super::Transporter;
use crate::io;
use crate::server_message::ServerMessageSender;
use crate::ServerManager;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io as tokio_io;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub struct Acceptor {
    pub server_manager: Arc<Mutex<ServerManager>>,
    pub server_message_sender: ServerMessageSender,
}

impl Acceptor {
    pub fn new(
        server_manager: Arc<Mutex<ServerManager>>,
        server_message_sender: ServerMessageSender,
    ) -> Self {
        Self {
            server_manager,
            server_message_sender,
        }
    }

    pub async fn accept(self, stream: TcpStream) {
        let id = self.server_manager.lock().await.next_id();
        let (transporter, transporter_handle) = Transporter::new(stream).await;
        let io_client = io::Client::new(
            io::ClientId(id),
            transporter,
            self.server_message_sender.clone(),
        )
        .await;

        let handle = io_client.clone().listen();

        self.server_manager
            .lock()
            .await
            .add_client(io_client.clone());

        let welcome = tokio::spawn(async move {
            io_client.welcome().await;
        });

        let _ = tokio::join!(welcome, transporter_handle, handle);
        println!("left");
    }

    pub async fn handle_incoming(self, incoming: tokio_io::Result<(TcpStream, SocketAddr)>) {
        match incoming {
            Ok((stream, _)) => self.accept(stream).await,
            Err(e) => {
                eprintln!("Error on new connection: {}", e);
            }
        }
    }
}
