use std::net;
use std::io::Read;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use ::world_store::ServerWorldStore;
use utils::thread;
use ::protocol::common::ClientInfo;

// Contains all the server state that the tcp handlers need to share.
pub struct TcpHandlerContext {
    pub connected_clients_lock: Arc<RwLock<HashMap<String, (ClientInfo, net::SocketAddr)>>>,
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
            let (mut stream, _) = tcp_listener.accept().unwrap();

            // TODO(aeidelson): Parse and process the protobuf.
            let mut result_str = String::new();
            stream.read_to_string(&mut result_str).unwrap();
            println!("Received from client: {}", result_str);
        }
    });

    (cancel_sender, server_tcp_port)
}

pub fn handle_connect(context: &TcpHandlerContext) {
}
