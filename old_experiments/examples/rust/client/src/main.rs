extern crate rps;

use rps::peer_connection::client;
use std::time::Duration;
use std::thread;

fn main() {
    let mut client = client::Client::new(client::ClientConfig {
        // This must be the same as the port the server broadcasts to.
        alive_broadcast_port: 8888,
    });

    client.start_discovering_servers();

    println!("Client should be discovering servers!");

    thread::sleep(Duration::from_secs(10));

    
    println!("Discovered servers: {:?}", client.discovered_servers());

    client.stop_discovering_servers();

    println!("Discovered servers (again): {:?}", client.discovered_servers());

    thread::sleep(Duration::from_secs(10));
}
