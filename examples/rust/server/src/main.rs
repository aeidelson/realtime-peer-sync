extern crate rps;

use std::str::FromStr;
use std::net::SocketAddr;
use rps::peer_connection::server;
use std::time::Duration;
use std::thread;

fn main() {
    let mut server = server::Server::new(server::ServerConfig {
        broadcast_address: SocketAddr::from_str("255.255.255.255:8888").unwrap(),
    });

    server.start();

    println!("Hello, world!");

    thread::sleep(Duration::from_secs(10));

    server.shutdown();

    thread::sleep(Duration::from_secs(10));
}
