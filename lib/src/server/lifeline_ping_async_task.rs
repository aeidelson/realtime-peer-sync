use std::time::Duration;
use std::io::Read;
use std::net::{SocketAddr, UdpSocket, IpAddr};
use std::thread as std_thread;
use std::str::FromStr;

use protobuf::Message;

use utils::thread;
use internal_protocol::gen::common::PublicServerInfo;
use internal_protocol::gen::server_lifeline_ping::ServerLifelinePing;

// Starts broadcasting the server availability.
pub fn start(
    // The udp port to broadcast to.
    borrowed_udp_broadcast_port: &u32,

    // The name of the server.
    borrowed_server_name: &str,

    // The tcp port the server is listening on for client requests.
    borrowed_server_tcp_port: &u32,
) -> thread::CancelSender {
    let udp_broadcast_port: u16 = borrowed_udp_broadcast_port.clone() as u16;
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
        let mut broadcast_message = ServerLifelinePing::new();
        {
            let public_server_info = broadcast_message.mut_public_info();
            public_server_info.set_name(server_name);
            public_server_info.set_tcp_port(server_tcp_port);
        }

        let broadcast_message_byte_vector  = broadcast_message.write_to_bytes().unwrap();
        let broadcast_message_bytes: &[u8] = broadcast_message_byte_vector.as_slice();

        // Broadcast the message on a loop.
        while !cancel_receiver.has_been_canceled() {
            println!("broadcasting!!");
            socket.send_to(&broadcast_message_bytes, broadcast_addr).unwrap();
            std_thread::sleep(Duration::from_millis(500));
        }
    });
}
