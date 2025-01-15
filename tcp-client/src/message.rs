use packet::ping::PingType;
use tokio::{io::BufReader, net::TcpStream, sync::mpsc::Sender};

use crate::{transporter, Event, State};

#[derive(Debug)]
pub enum Message {
    Connected,
    Disconnected,
    Ping,
}

impl Message {
    pub async fn handle(
        self,
        output: &mut Sender<Event>,
        stream: &mut BufReader<&mut TcpStream>,
    ) -> Option<State> {
        let result = match self {
            Message::Ping => {
                let packet = packet::ping::create_packet(PingType::Ping);
                transporter::send_packet(stream, &packet).await.err()
            }
            _ => None,
        };
        match result {
            Some(err) => {
                let _ = output.send(Event::Disconnected).await;
                eprintln!("an error occured whilst handling message: {err:#?}");
                Some(State::Disconnected)
            }
            _ => None,
        }
    }
}
