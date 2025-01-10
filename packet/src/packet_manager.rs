use std::io::Cursor;

use crate::packet::PacketType;
use prost::Message;

// Include the `authenticate` module, which is generated from packet.proto.
include!(concat!(env!("OUT_DIR"), "/packets.packet.rs"));

pub fn create<M: Message>(packet_type: PacketType, message: M) -> Packet {
    let mut packet = Packet::default();
    packet.set_packet_type(packet_type);

    packet.value.reserve(message.encoded_len());
    message.encode(&mut packet.value).unwrap();

    packet
}

pub fn serialize(packet: &Packet) -> Vec<u8> {
    let mut buf = Vec::with_capacity(packet.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    packet.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize(buf: &[u8]) -> Result<Packet, prost::DecodeError> {
    Packet::decode(&mut Cursor::new(buf))
}
