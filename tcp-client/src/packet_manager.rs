use tokio::sync::mpsc::Sender;

use crate::Event;

pub async fn handle_packet(output: &mut Sender<Event>, content: String) {
    let packet = packet::deserialize(&trunc_message(content.into()));

    match packet {
        Ok(packet) => {
            let _ = output.send(Event::PacketReceived(packet)).await;
        }
        Err(err) => {
            eprintln!("Cannot deserialize packet: {err:#?}");
        }
    };
}

fn trunc_message(mut message: Vec<u8>) -> Vec<u8> {
    if message.last().is_some_and(|byte| byte == &b'\n') {
        message.pop();
        if message.last().is_some_and(|byte| byte == &b'\r') {
            message.pop();
        }
    }
    message
}
