use tokio::{
    io::AsyncBufReadExt,
    net::TcpStream,
    select,
    sync::mpsc::{self, Receiver, Sender},
};

use crate::{
    client::{handle_message, handle_response},
    Event, Message, MessageTransporter,
};

#[derive(Debug)]
pub enum State {
    Disconnected,
    Connected(TcpStream, Receiver<Message>),
}

impl State {
    async fn handle_connected(
        output: &mut Sender<Event>,
        stream: &mut TcpStream,
        input: &mut Receiver<Message>,
    ) -> Option<State> {
        let mut stream = tokio::io::BufReader::new(stream);
        let mut content = String::new();

        select! {
            response = stream.read_line(&mut content) => {
                handle_response(output, content, response).await
            }
            response = input.recv() => {
                handle_message(output, &mut stream, response).await
            }
        }
    }

    async fn handle_disconnected(&mut self, output: &mut Sender<Event>) {
        const SERVER: &str = "127.0.0.1:3000";

        match TcpStream::connect(SERVER).await {
            Ok(server) => {
                let (sender, receiver) = mpsc::channel(100);
                let _ = output
                    .send(Event::Connected(MessageTransporter(sender)))
                    .await;
                *self = State::Connected(server, receiver);
            }
            Err(_) => {
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

                let _ = output.send(Event::Disconnected).await;
            }
        }
    }

    pub async fn handle(&mut self, output: &mut Sender<Event>) {
        match self {
            State::Disconnected => {
                self.handle_disconnected(output).await;
            }
            State::Connected(stream, input) => {
                let new_state = Self::handle_connected(output, stream, input).await;
                if let Some(new_state) = new_state {
                    *self = new_state;
                }
            }
        }
    }
}
