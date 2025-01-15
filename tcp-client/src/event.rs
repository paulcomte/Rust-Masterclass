use packet::Packet;

use crate::transporter::MessageTransporter;

#[derive(Debug, Clone)]
pub enum Event {
    Connected(MessageTransporter),
    Disconnected,
    PacketReceived(Packet),
}
