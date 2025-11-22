mod server;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let port: usize = if args.len() >= 2 {
        // First argument is the path of the executable.
        args[1].parse().expect("Invalid port")
    } else {
        // Let the OS pick an available port.
        0
    };

    match server::Server::new(port) {
        Ok(mut s) => s.run(),
        Err(e) => eprintln!("Failed to create server: {e:?}"),
    }
}
