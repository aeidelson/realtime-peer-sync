use std::net;
use std::time;

use super::DiscoveredServerInfo;

// Blocks for the provided ammount of time, listening on the provided port for servers broadcasting
// their availability.
pub fn start(discovery_port: &u32, wait_time: time::Duration) -> Vec<DiscoveredServerInfo> {
    let local_addr = net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), discovery_port.clone() as u16);
    let discovery_socket = net::UdpSocket::bind(local_addr).unwrap();
    discovery_socket.set_nonblocking(true).unwrap();

    let start_time = time::Instant::now();
    while start_time.elapsed() >= wait_time {
        // TODO(aeidelson): Read from socket if there is anything to read, otherwise wait.
    }

    vec![]
}
