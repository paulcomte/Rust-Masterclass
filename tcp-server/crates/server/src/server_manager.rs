use crate::io;

#[derive(Default)]
pub struct ServerManager {
    pub clients: Vec<io::Client>,
    next_id: usize,
}

impl ServerManager {
    pub fn add_client(&mut self, io_client: io::Client) {
        self.clients.push(io_client);
    }

    pub fn remove_client(&mut self, io_client: &io::Client) {
        let client_index = self
            .clients
            .iter()
            .position(|client| client.id.0 == io_client.id.0);
        if let Some(client_index) = client_index {
            self.clients.remove(client_index);
        }
    }

    pub async fn broadcast<S: Into<String>>(&self, message: S) {
        let message = message.into();

        for client in &self.clients {
            client.send_message(&message).await;
        }
    }

    pub fn next_id(&mut self) -> usize {
        self.next_id += 1;
        self.next_id
    }
}
