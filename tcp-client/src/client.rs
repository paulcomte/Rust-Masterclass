use std::io;

use tokio::{io::BufReader, net::TcpStream, sync::mpsc::Sender};

use crate::{packet_manager::handle_packet, Event, Message, State};

pub async fn handle_message(
    output: &mut Sender<Event>,
    stream: &mut BufReader<&mut TcpStream>,
    response: Option<Message>,
) -> Option<State> {
    match response {
        Some(message) => message.handle(output, stream).await,
        _ => None,
    }
}

pub async fn handle_response(
    output: &mut Sender<Event>,
    content: String,
    response: io::Result<usize>,
) -> Option<State> {
    match response {
        Ok(_) => {
            handle_packet(output, content).await;
            None
        }

        Err(err) => {
            eprintln!("Cannot read received buffer: {err:#?}");
            let _ = output.send(Event::Disconnected).await;
            Some(State::Disconnected)
        }
    }
}

pub fn connect() -> Subscription<Event> {
    struct Connect;

    subscription::channel(
        std::any::TypeId::of::<Connect>(),
        100,
        |mut output| async move {
            let mut state = State::Disconnected;

            loop {
                state.handle(&mut output).await;
            }
        },
    )
}
