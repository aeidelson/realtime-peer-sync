use std::mem;
use std::net;
use std::time;
use std::thread;
use std::cmp;
use std::collections::HashMap;

use bincode;

use protocol::common::PublicServerInfo;
use protocol::server_lifeline_ping::ServerLifelinePing;
use super::DiscoveredServerInfo;

// Blocks for the provided ammount of time, listening on the provided port for servers broadcasting
// their availability.
pub fn start(discovery_port: &u16, wait_time: time::Duration) -> Vec<DiscoveredServerInfo> {
    let local_addr = net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), discovery_port.clone() as u16);
    let discovery_socket = net::UdpSocket::bind(local_addr).unwrap();
    discovery_socket.set_nonblocking(true).unwrap();

    let start_time = time::Instant::now();

    let mut results = HashMap::new();

    let mut buf = [0; 10000];
    while start_time.elapsed() <= wait_time {
        let (read, ping_source) = match discovery_socket.recv_from(&mut buf) {
            Ok(result) => result,
            _ => continue,
        };

        // Process and store
        // TODO: Handle error here so bad packet doesn't crash.
        // TODO: Make sure we don't take a slice bigger than buf.
        let ping: ServerLifelinePing = bincode::rustc_serialize::decode(&buf[..read]).unwrap();
        //let ping: ServerLifelinePing = parse_from_bytes(&buf[..read]).unwrap();

        // Construct a DiscoveredServerInfo
        let mut tcp_addr = ping_source.clone();
        tcp_addr.set_port(ping.public_info.tcp_port);

        results.insert(ping_source, DiscoveredServerInfo {
            server_name: ping.public_info.name.clone(),
            // FIX
            tcp_server_location: tcp_addr,
        });

        thread::sleep(time::Duration::from_millis(50));
    }

    results.into_iter().map(| (k, v) | v).collect()
}
