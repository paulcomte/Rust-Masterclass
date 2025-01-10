//! --- Server Module ---
//!
//! A TCP server implementation

mod io;
mod packet_handler;
mod server;
mod server_manager;
mod server_message;
mod tcp;

pub use io::*;
pub use packet_handler::PacketHandler;
pub use server::Server;
pub use server_manager::ServerManager;
pub(crate) use server_message::ServerMessage;
pub use tcp::*;

/// Give packet vision over other crates
pub use packet;

use std::sync::Arc;
use tokio::{sync::Mutex, task::JoinHandle};

pub async fn run_server<S: Into<String>>(
    target_host: S,
    port: u16,
    packet_handler: impl PacketHandler + 'static,
) -> (Arc<Mutex<ServerManager>>, JoinHandle<()>) {
    let server_manager = Arc::new(Mutex::new(ServerManager::default()));
    let target_host = target_host.into();

    let local_server_manager = server_manager.clone();
    let server_handle = tokio::spawn(async move {
        let mut server = Server::new(target_host, port, local_server_manager);
        server.run(packet_handler).await;
    });

    (server_manager, server_handle)
}
