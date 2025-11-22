use super::Result;
use super::broadcast::broadcast;
use std::io::{BufReader, BufWriter, prelude::*};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

pub struct Client {
    pub(crate) name: String,
    pub(crate) stream: TcpStream,
}

impl Client {
    pub fn new(name: String, stream: TcpStream) -> Self {
        Self { name, stream }
    }

    pub fn try_clone(&self) -> Result<Self> {
        let name = self.name.clone();
        let stream = self.stream.try_clone()?;

        Ok(Self { name, stream })
    }
}

pub fn handle(client: Client, clients: Arc<Mutex<Vec<Client>>>) -> Result<()> {
    let mut writer = BufWriter::new(client.stream.try_clone()?);
    writer.write_all("Connected to server\n".as_bytes())?;

    // Dropping forces the writer to flush.
    drop(writer);

    let mut reader = BufReader::new(client.stream.try_clone()?);

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => {
                // Client has closed the connection.
                break;
            }
            Ok(_) => {
                println!("Received from {}: {}", client.name, line.trim());
                let msg = format!("{}: {line}", client.name);
                if let Err(e) = broadcast(&client.name, clients.clone(), &msg) {
                    eprintln!("Broadcast failed for {}: {e:?}", client.name);
                }
            }
            Err(e) => {
                eprintln!("Line parsing failed for {}: {e:?}", client.name);
            }
        }
    }

    Ok(())
}

pub fn remove(client: &str, clients: Arc<Mutex<Vec<Client>>>) -> Result<()> {
    let mut clients = clients.lock()?;
    clients.retain(|c| c.name != client);

    Ok(())
}
