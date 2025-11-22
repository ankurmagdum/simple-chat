use super::Result;
use super::client::Client;

use std::io::{BufWriter, prelude::*};
use std::sync::{Arc, Mutex};

pub fn broadcast(sender: &str, clients: Arc<Mutex<Vec<Client>>>, msg: &str) -> Result<()> {
    let mut clients = clients.lock()?;

    for client in clients.iter_mut() {
        if &client.name == sender {
            continue;
        }

        if let Ok(stream) = client.stream.try_clone() {
            let mut writer = BufWriter::new(stream);

            println!("Broadcasting to {}", client.name);
            if let Err(e) = writer.write_all(msg.as_bytes()) {
                eprintln!("Write failed for {}: {e:?}", &client.name);
            }
        }
    }

    Ok(())
}
