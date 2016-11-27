use std::net;
use std::io::Read;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use bincode;

use utils::thread;
use ::world_store::ServerWorldStore;
use super::common::ConnectedClientInfo;
use ::protocol::common::ClientInfo;
use ::protocol::message_wrapper::{ClientServerMessage, ClientServerMessagePayload};
use ::protocol::connect::{ClientServerConnect};


// Contains all the server state that the tcp handlers need to share.
#[derive(Clone)]
pub struct TcpHandlerContext {
    pub client_message_udp_port: u16,
    pub connected_clients_lock: Arc<RwLock<HashMap<String, ConnectedClientInfo>>>,
    pub store_lock: Arc<RwLock<ServerWorldStore>>,
}

// Starts a thread to listen for and handle tcp requests.  In addition to the cancel sender, the
// function returns server's tcp port (to broadcast to clients).
pub fn start(context: TcpHandlerContext) -> (thread::CancelSender, u16) {
    let tcp_listener = net::TcpListener::bind("0.0.0.0:0").unwrap();
    let server_tcp_port = tcp_listener.local_addr().unwrap().port();

    let cancel_sender = thread::spawn_cancelable(move |cancel_receiver: thread::CancelReceiver| {
        while !cancel_receiver.has_been_canceled() {
            // Will block until there's something to process.
            let (mut stream, remote_addr) = tcp_listener.accept().unwrap();

            // TODO(aeidelson): For now, each tcp connection represents a message. This could be
            // more efficient if each connection represented a client, and messages were framed in
            // some way.
            let mut client_message: Vec<u8> = vec![];
            stream.read_to_end(&mut client_message);
            let decoded: ClientServerMessage =
                bincode::rustc_serialize::decode(&client_message[..]).unwrap();

            // Call the correct handler based on the message from the client.
            match decoded.payload {
                ClientServerMessagePayload::Connect(connect_message) =>
                    handle_connect(&context, &remote_addr, &decoded.client_info, &connect_message),
                _ => panic!("Unhandled payload type"),
            }

        }
    });

    (cancel_sender, server_tcp_port)
}

pub fn handle_connect(
    context: &TcpHandlerContext,
    remote_addr: &net::SocketAddr,
    client_info: &ClientInfo,
    connect_message: &ClientServerConnect,
) {
    // Construct the address that the client is listening on.
    let client_udp_addr = net::SocketAddr::new(remote_addr.ip(), context.client_message_udp_port);
    let mut connected_clients = context.connected_clients_lock.write().unwrap();
    connected_clients.insert(
        client_info.public_info.client_id.clone(),
        ConnectedClientInfo {
            client_info: client_info.clone(),
            socket_addr: client_udp_addr.clone(),
            acked_world_version: 0,
        });

    println!("> Client connected!");
}
