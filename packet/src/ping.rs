use std::io::Cursor;

use prost::Message;

pub use ping::PingType;

use crate::{packet::PacketType, packet_manager, Packet};

// Include the `ping` module, which is generated from ping.proto.
include!(concat!(env!("OUT_DIR"), "/packets.ping.rs"));

pub fn create(ping_type: PingType) -> Ping {
    Ping {
        ping_type: ping_type as i32,
    }
}

pub fn create_packet(ping_type: PingType) -> Packet {
    packet_manager::create(PacketType::Ping, serialize(&create(ping_type)))
}

pub fn serialize(ping: &Ping) -> Vec<u8> {
    ping.encode_to_vec()
}

pub fn deserialize(buf: &[u8]) -> Result<Ping, prost::DecodeError> {
    Ping::decode(&mut Cursor::new(buf))
}
