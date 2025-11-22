mod broadcast;
mod client;
mod error;

use broadcast::broadcast;
use client::Client;
use error::ServerError;

use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub type Result<T> = std::result::Result<T, ServerError>;

pub struct Server {
    listener: TcpListener,
    clients: Arc<Mutex<Vec<Client>>>,
    next_id: usize,
}

impl Server {
    pub fn new(port: usize) -> Result<Self> {
        let listener = TcpListener::bind(format!("0.0.0.0:{port}"))?;
        println!("Server listening on {:?}", listener.local_addr()?);
        let clients = Arc::new(Mutex::new(Vec::new()));
        let next_id = 0;

        Ok(Self {
            listener,
            clients,
            next_id,
        })
    }

    pub fn run(&mut self) {
        for stream in self.listener.incoming() {
            if let Err(e) = self.add_new_client(stream) {
                eprintln!("Connection error: {e:?}");
            } else {
                self.next_id += 1;
            }
        }
    }

    fn add_new_client(&self, stream: std::io::Result<TcpStream>) -> Result<()> {
        let stream = stream?;
        println!("Client connected {:?}", stream.peer_addr()?);

        let client_name = format!("client{:}", self.next_id);
        let client = Client::new(client_name.clone(), stream);

        let mut clients_guard = self.clients.lock()?;
        clients_guard.push(client.try_clone()?);
        let clients = self.clients.clone();

        thread::spawn(move || {
            if let Err(e) = manage_client(client, clients) {
                eprintln!("Error managing {client_name}: {e:?}");
            }
        });

        Ok(())
    }
}

fn manage_client(client: Client, clients: Arc<Mutex<Vec<Client>>>) -> Result<()> {
    let client_name = client.name.clone();

    let broadcast_msg = &format!("{} has joined\n", client_name);
    broadcast(&client.name, clients.clone(), broadcast_msg)?;

    client::handle(client, clients.clone())?;
    println!("{client_name} disconnected");

    // Remove the client from the list.
    client::remove(&client_name, clients.clone())?;

    let broadcast_msg = &format!("{} has left\n", client_name);
    broadcast(&client_name, clients, broadcast_msg)?;

    Ok(())
}
