use std::sync::Arc;

use crate::{io, ServerManager};
use packet::Packet;
use tokio::sync::Mutex;

/// PacketHandler trait
///
/// The function handle is called when the ServerMessage::HandlePacket case is matched
pub trait PacketHandler: Send {
    fn handle(
        &mut self,
        server_manager: Arc<Mutex<ServerManager>>,
        io_client: io::Client,
        packet: Packet,
    ) -> impl std::future::Future<Output = ()> + Send;
}
