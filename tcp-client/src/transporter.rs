use std::io;

use packet::Packet;
use tokio::{
    io::{AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::mpsc::Sender,
};

use crate::Message;

#[derive(Debug, Clone)]
pub struct MessageTransporter(pub Sender<Message>);

pub async fn send_packet(
    stream: &mut BufReader<&mut TcpStream>,
    packet: &Packet,
) -> io::Result<usize> {
    let mut message = packet::serialize(packet);
    message.extend_from_slice(b"\r\n");
    stream.write(&message).await
}
