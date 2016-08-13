use std::net;
use std::io::Read;

use utils::thread;

// Starts a thread to listen for and handle tcp requests.  In addition to the cancel sender, the
// function returns server's tcp port (to broadcast to clients).
pub fn start() -> (thread::CancelSender, u32) {
    let tcp_listener = net::TcpListener::bind("0.0.0.0:0").unwrap();
    let server_tcp_port = tcp_listener.local_addr().unwrap().port() as u32;

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
