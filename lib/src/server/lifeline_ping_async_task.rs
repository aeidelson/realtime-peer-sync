use std::time::Duration;
use std::io::Read;
use std::net::{SocketAddr, UdpSocket, IpAddr};
use std::thread as std_thread;
use std::str::FromStr;

use bincode;

use utils::thread;
use protocol::common::PublicServerInfo;
use protocol::server_lifeline_ping::ServerLifelinePing;

// Starts broadcasting the server availability.
pub fn start(
    // The udp port to broadcast to.
    borrowed_udp_broadcast_port: &u16,

    // The name of the server.
    borrowed_server_name: &str,

    // The tcp port the server is listening on for client requests.
    borrowed_server_tcp_port: &u16,
) -> thread::CancelSender {
    let udp_broadcast_port: u16 = borrowed_udp_broadcast_port.clone();
    let server_name: String = borrowed_server_name.to_string();
    let server_tcp_port = borrowed_server_tcp_port.clone();

    return thread::spawn_cancelable(move |cancel_receiver: thread::CancelReceiver| {
        // Setup the udp socket to broadcast from.
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        socket.set_broadcast(true).unwrap();
        // TODO(aeidelson): We should remove this when we properly merge the client and server.
        socket.set_multicast_loop_v4(true).unwrap();

        // Build the address we want to broadcast to.
        let broadcast_addr = SocketAddr::new(
            IpAddr::from_str("255.255.255.255").unwrap(),
            udp_broadcast_port,
        );

        // Build the message to broadcast.
        let broadcast_message = ServerLifelinePing {
            public_info: PublicServerInfo {
                tcp_port: server_tcp_port,
                name: server_name,
            },
        };

        let broadcast_message_byte_vector =
            bincode::rustc_serialize::encode(&broadcast_message, bincode::SizeLimit::Infinite).unwrap();
        let broadcast_message_bytes: &[u8] = broadcast_message_byte_vector.as_slice();

        // Broadcast the message on a loop.
        while !cancel_receiver.has_been_canceled() {
            println!("broadcasting!!");
            socket.send_to(&broadcast_message_bytes, broadcast_addr).unwrap();
            std_thread::sleep(Duration::from_millis(500));
        }
    });
}
