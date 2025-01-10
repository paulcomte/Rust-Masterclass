use std::sync::Arc;

use server::{packet::packet::PacketType, Client, ServerManager};
use tokio::{sync::Mutex, task::JoinHandle};

pub async fn run_server<S: Into<String>>(
    target_host: S,
    port: u16,
) -> (Arc<Mutex<ServerManager>>, JoinHandle<()>) {
    let packet_handler = PacketHandler;
    server::run_server(target_host, port, packet_handler).await
}

pub struct PacketHandler;

impl server::PacketHandler for PacketHandler {
    async fn handle(
        &mut self,
        _server_manager: Arc<Mutex<ServerManager>>,
        io_client: Client,
        packet: server::packet::Packet,
    ) {
        match packet.packet_type() {
            PacketType::Ping => {
                eprintln!("ping packet received!");
            }
            _ => {
                io_client.send_message("See you next time!").await;
                io_client.stop().await;
                return;
            }
        }
    }
}
