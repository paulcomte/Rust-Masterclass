mod server_message_handler;
mod server_message_sender;

pub use server_message_handler::ServerMessageHandler;
pub use server_message_sender::ServerMessageSender;

use crate::io;

pub enum ServerMessage {
    HandlePacket(io::Client, Vec<u8>),
    Disconnect(io::Client),
}
